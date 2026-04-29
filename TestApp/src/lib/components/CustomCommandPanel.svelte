<script lang="ts">
  import { sendCustomCommand } from '../services/commands';

  let command = '';
  let status = '[READY] Esperando comando...';
  let statusType: 'idle' | 'success' | 'error' = 'idle';
  let loading = false;

  async function handleSubmit() {
    const trimmed = command.trim();

    if (!trimmed) {
      status = '[ERROR] Escribí un comando';
      statusType = 'error';
      return;
    }

    loading = true;
    statusType = 'idle';

    try {
      const response = await sendCustomCommand(trimmed);
      status = `[OK] ${response}`;
      statusType = 'success';
      command = '';
    } catch (error) {
      status = `[ERROR] ${String(error)}`;
      statusType = 'error';
    } finally {
      loading = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      event.preventDefault();
      handleSubmit();
    }
  }
</script>

<section class="panel custom-command-panel">
  <div class="panel-title">Send Custom Command</div>

  <div class="custom-command-terminal">
    <p class:success={statusType === 'success'} class:error={statusType === 'error'}>
      {status}
    </p>
  </div>

  <div class="command-form">
    <input
      type="text"
      bind:value={command}
      placeholder="........"
      on:keydown={handleKeydown}
      disabled={loading}
    />

    <button type="button" on:click={handleSubmit} disabled={loading}>
      {#if loading}
        Enviando...
      {:else}
        Send
      {/if}
    </button>
  </div>
</section>