<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  interface Task {
    id: string;
    text: string;
    done: boolean;
    notify_at: string | null;
    notified: boolean;
    created_at: string;
  }

  let tasks: Task[] = $state([]);
  let inputValue = $state("");
  let isLoading = $state(true);

  async function loadTasks() {
    try {
      tasks = await invoke<Task[]>("get_tasks");
    } catch (e) {
      console.error("Failed to load tasks:", e);
    } finally {
      isLoading = false;
    }
  }

  async function addTask() {
    const text = inputValue.trim();
    if (!text) return;

    try {
      const task = await invoke<Task>("add_task", { input: text });
      tasks = [...tasks, task];
      inputValue = "";
    } catch (e) {
      console.error("Failed to add task:", e);
    }
  }

  async function toggleTask(id: string) {
    try {
      const updated = await invoke<Task>("toggle_task", { id });
      tasks = tasks.map((t) => (t.id === id ? updated : t));

      // Remove done tasks after animation
      if (updated.done) {
        setTimeout(() => {
          tasks = tasks.filter((t) => t.id !== id);
          invoke("delete_task", { id });
        }, 600);
      }
    } catch (e) {
      console.error("Failed to toggle task:", e);
    }
  }

  async function clearAll() {
    try {
      await invoke("clear_all_tasks");
      tasks = [];
    } catch (e) {
      console.error("Failed to clear tasks:", e);
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Enter" && !event.ctrlKey && !event.metaKey) {
      event.preventDefault();
      addTask();
    }
    if (event.key === "Enter" && (event.ctrlKey || event.metaKey)) {
      event.preventDefault();
      clearAll();
    }
    if (event.key === "Escape") {
      event.preventDefault();
      getCurrentWindow().minimize();
    }
  }

  function handleDragStart(event: MouseEvent) {
    if ((event.target as HTMLElement).closest(".title-btn")) return;
    getCurrentWindow().startDragging();
  }

  function minimizeWindow() {
    getCurrentWindow().minimize();
  }

  function closeWindow() {
    getCurrentWindow().hide();
  }

  // Load tasks on mount
  $effect(() => {
    loadTasks();
  });
</script>

<div class="app-container" role="application">
  <!-- Custom Title Bar -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="title-bar" onmousedown={handleDragStart}>
    <span class="title-text">QuickTask</span>
    <div class="title-actions">
      <button class="title-btn minimize" onclick={minimizeWindow} aria-label="Minimize">
        <svg width="10" height="1" viewBox="0 0 10 1"><rect width="10" height="1" fill="currentColor"/></svg>
      </button>
      <button class="title-btn close" onclick={closeWindow} aria-label="Close">
        <svg width="10" height="10" viewBox="0 0 10 10">
          <line x1="0" y1="0" x2="10" y2="10" stroke="currentColor" stroke-width="1.2"/>
          <line x1="10" y1="0" x2="0" y2="10" stroke="currentColor" stroke-width="1.2"/>
        </svg>
      </button>
    </div>
  </div>

  <!-- Input Area -->
  <div class="input-area">
    <span class="input-icon">+</span>
    <input
      type="text"
      class="task-input"
      placeholder="Add a quick task..."
      bind:value={inputValue}
      onkeydown={handleKeydown}
      autofocus
    />
  </div>

  <div class="divider"></div>

  <!-- Task List -->
  <div class="task-list">
    {#if isLoading}
      <div class="empty-state">Loading...</div>
    {:else if tasks.length === 0}
      <div class="empty-state">
        <span class="empty-icon">○</span>
        <span>No tasks yet</span>
      </div>
    {:else}
      {#each tasks as task (task.id)}
        <div class="task-item" class:done={task.done}>
          <button
            class="checkbox"
            class:checked={task.done}
            onclick={() => toggleTask(task.id)}
            aria-label={task.done ? "Mark as incomplete" : "Mark as complete"}
          >
            {#if task.done}
              <svg width="12" height="12" viewBox="0 0 12 12">
                <polyline points="2,6 5,9 10,3" stroke="currentColor" stroke-width="1.5" fill="none"/>
              </svg>
            {/if}
          </button>
          <span class="task-text">{task.text}</span>
          {#if task.notify_at}
            <span class="time-badge">
              <svg width="10" height="10" viewBox="0 0 10 10" class="clock-icon">
                <circle cx="5" cy="5" r="4" stroke="currentColor" stroke-width="1" fill="none"/>
                <line x1="5" y1="5" x2="5" y2="2.5" stroke="currentColor" stroke-width="1"/>
                <line x1="5" y1="5" x2="7" y2="5" stroke="currentColor" stroke-width="1"/>
              </svg>
              {task.notify_at}
            </span>
          {/if}
        </div>
      {/each}
    {/if}
  </div>

  <!-- Footer -->
  <div class="footer">
    <span class="hint">Enter to add · Ctrl+Enter to clear all · Esc to minimize</span>
  </div>
</div>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(body) {
    background: transparent;
    font-family: 'SF Mono', 'Cascadia Code', 'Fira Code', 'Consolas', monospace;
    color: #e0e0e0;
    overflow: hidden;
    user-select: none;
    -webkit-user-select: none;
  }

  .app-container {
    background: rgba(10, 10, 10, 0.95);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 12px;
    height: 100vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  /* Title Bar */
  .title-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 16px;
    cursor: grab;
    flex-shrink: 0;
  }

  .title-bar:active {
    cursor: grabbing;
  }

  .title-text {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: rgba(255, 255, 255, 0.3);
  }

  .title-actions {
    display: flex;
    gap: 8px;
  }

  .title-btn {
    width: 24px;
    height: 24px;
    border: none;
    background: transparent;
    color: rgba(255, 255, 255, 0.25);
    cursor: pointer;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s ease;
  }

  .title-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    color: rgba(255, 255, 255, 0.6);
  }

  .title-btn.close:hover {
    background: rgba(255, 80, 80, 0.2);
    color: rgba(255, 100, 100, 0.8);
  }

  /* Input Area */
  .input-area {
    display: flex;
    align-items: center;
    padding: 4px 16px 12px;
    gap: 10px;
    flex-shrink: 0;
  }

  .input-icon {
    font-size: 16px;
    color: rgba(255, 255, 255, 0.2);
    font-weight: 300;
    flex-shrink: 0;
  }

  .task-input {
    flex: 1;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 8px;
    padding: 10px 14px;
    font-family: inherit;
    font-size: 13px;
    color: #e0e0e0;
    outline: none;
    transition: all 0.2s ease;
  }

  .task-input::placeholder {
    color: rgba(255, 255, 255, 0.2);
  }

  .task-input:focus {
    border-color: rgba(255, 255, 255, 0.12);
    background: rgba(255, 255, 255, 0.06);
  }

  .divider {
    height: 1px;
    background: rgba(255, 255, 255, 0.04);
    margin: 0 16px;
    flex-shrink: 0;
  }

  /* Task List */
  .task-list {
    flex: 1;
    overflow-y: auto;
    padding: 8px 0;
    scrollbar-width: thin;
    scrollbar-color: rgba(255, 255, 255, 0.08) transparent;
  }

  .task-list::-webkit-scrollbar {
    width: 4px;
  }

  .task-list::-webkit-scrollbar-track {
    background: transparent;
  }

  .task-list::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.08);
    border-radius: 2px;
  }

  .task-item {
    display: flex;
    align-items: center;
    padding: 8px 16px;
    gap: 10px;
    transition: all 0.3s ease;
    animation: slideIn 0.2s ease-out;
  }

  .task-item:hover {
    background: rgba(255, 255, 255, 0.02);
  }

  .task-item.done {
    animation: fadeOut 0.5s ease-out forwards;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(-8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes fadeOut {
    0% {
      opacity: 1;
      transform: translateX(0);
    }
    100% {
      opacity: 0;
      transform: translateX(20px);
    }
  }

  .checkbox {
    width: 16px;
    height: 16px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 4px;
    background: transparent;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    color: rgba(255, 255, 255, 0.6);
    transition: all 0.15s ease;
    flex-shrink: 0;
    padding: 0;
  }

  .checkbox:hover {
    border-color: rgba(255, 255, 255, 0.3);
    background: rgba(255, 255, 255, 0.05);
  }

  .checkbox.checked {
    border-color: rgba(255, 255, 255, 0.1);
    background: rgba(255, 255, 255, 0.08);
  }

  .task-text {
    flex: 1;
    font-size: 13px;
    line-height: 1.4;
    color: rgba(255, 255, 255, 0.75);
  }

  .done .task-text {
    text-decoration: line-through;
    color: rgba(255, 255, 255, 0.2);
  }

  .time-badge {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 11px;
    color: rgba(255, 255, 255, 0.35);
    background: rgba(255, 255, 255, 0.04);
    padding: 2px 8px;
    border-radius: 4px;
    flex-shrink: 0;
  }

  .clock-icon {
    opacity: 0.6;
  }

  /* Empty State */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 40px 16px;
    color: rgba(255, 255, 255, 0.15);
    font-size: 13px;
  }

  .empty-icon {
    font-size: 24px;
    opacity: 0.5;
  }

  /* Footer */
  .footer {
    padding: 8px 16px;
    border-top: 1px solid rgba(255, 255, 255, 0.04);
    flex-shrink: 0;
  }

  .hint {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.12);
    letter-spacing: 0.02em;
  }
</style>
