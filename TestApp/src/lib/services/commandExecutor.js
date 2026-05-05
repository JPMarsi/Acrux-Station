import { sendCustomCommand } from './commands';
import { resetTelemetrySession } from './telemetry';
import { commandConsole } from '../stores/commandConsole';

/**
 * @param {string} command
 */
function isResetCommand(command) {
  const normalized = command.trim().toUpperCase();
  return normalized === 'RESET' || normalized === 'RESET APP';
}

/**
 * @param {string} command
 */
export async function executeProtocolCommand(command) {
  const trimmed = command.trim();

  if (!trimmed) {
    commandConsole.push('[ERROR] Comando vacío', 'error');
    commandConsole.setLoading(false);
    return;
  }

  commandConsole.push(`[WAIT] Enviando: ${trimmed}`, 'idle');
  commandConsole.setLoading(true);

  try {
    if (isResetCommand(trimmed)) {
      await resetTelemetrySession();
      commandConsole.reset();
      commandConsole.push('[OK] App reiniciada: serial cerrado, CSV nuevo y graficos limpios', 'success');
      return;
    }

    const response = await sendCustomCommand(trimmed);
    commandConsole.push(`[OK] ${response}`, 'success');
  } catch (error) {
    commandConsole.push(`[ERROR] ${String(error)}`, 'error');
  } finally {
    commandConsole.setLoading(false);
  }
}
