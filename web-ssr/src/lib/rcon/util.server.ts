import { execute } from './rcon.server';

// Executes a server ping command and returns true if the server responds with 'Pong'.
export const livenessCheck = () => execute('ping') == 'Pong';

// Returns the players currently active on the server.
export const players = () => {
	const players: string = execute('ShowPlayers');

	return players
		.split('\n')
		.filter((l) => !!l)
		.map((l) => l.split(','))
		.map(([name, uid, steamId]) => ({ name, uid, steamId }));
};

export const broadcast = (msg: string) => {
	if (msg.length > 100) {
		throw new Error('Message too long');
	}

	execute(`Broadcast ${msg}`);
};
