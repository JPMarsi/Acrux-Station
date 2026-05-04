<script lang="ts">
  import { tick } from 'svelte';
  import { commandConsole } from '../stores/commandConsole';
  import { executeProtocolCommand } from '../services/commandExecutor';

  let command = '';
  let terminalEl: HTMLDivElement;

  async function handleSubmit() {
    const trimmed = command.trim();

    if (!trimmed) {
      commandConsole.push('[ERROR] Escribí un comando de protocolo', 'error');
      return;
    }

    await executeProtocolCommand(trimmed);
    command = '';
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      event.preventDefault();
      handleSubmit();
    }
  }

  $: if ($commandConsole.lines.length > 0) {
    scrollToBottom();
  }

  async function scrollToBottom() {
    await tick();

    if (terminalEl) {
      terminalEl.scrollTop = terminalEl.scrollHeight;
    }
  }
</script>

<section class="panel custom-command-panel">
  <div class="panel-title">Send Custom Command</div>

  <div class="custom-command-terminal" bind:this={terminalEl}>
    {#each $commandConsole.lines as line}
      <p
        class:success={line.type === 'success'}
        class:error={line.type === 'error'}
      >
        {line.message}
      </p>
    {/each}
  </div>

  <div class="command-form">
    <input
      type="text"
      bind:value={command}
      placeholder="CMD,1234,CX,ON"
      on:keydown={handleKeydown}
      disabled={$commandConsole.loading}
    />

    <button
      type="button"
      on:click={handleSubmit}
      disabled={$commandConsole.loading}
    >
      {#if $commandConsole.loading}
        Enviando...
      {:else}
        Send
      {/if}
    </button>
  </div>
</section>