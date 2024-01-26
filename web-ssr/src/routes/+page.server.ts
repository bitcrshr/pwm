import { execute } from '$lib/rcon/rcon.server';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const playersStr = execute('ShowPlayers');

	const players = playersStr
		.split('\n')
		.slice(1)
		.filter((l) => !!l)
		.map((l) => l.split(','))
		.map(([name, uid, steamId]) => ({ name, uid, steamId }));

	console.log(players);

	return { players };
};
