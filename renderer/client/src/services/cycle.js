import { evoSimHost } from '../config';

export async function get_cycle(simulation_id, cycle_id) {
    return await fetch(`${evoSimHost}/simulations/${simulation_id}/cycles/${cycle_id}`);
}