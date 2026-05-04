import { sendCustomCommand } from './commands';
import { commandConsole } from '../stores/commandConsole';

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
    const response = await sendCustomCommand(trimmed);
    commandConsole.push(`[OK] ${response}`, 'success');
  } catch (error) {
    commandConsole.push(`[ERROR] ${String(error)}`, 'error');
  } finally {
    commandConsole.setLoading(false);
  }
}