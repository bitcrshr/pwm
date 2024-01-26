import { dlopen, FFIType, suffix } from 'bun:ffi';

import { PRIVATE_RCON_HOST, PRIVATE_RCON_PASSWORD, PRIVATE_RCON_PORT } from '$env/static/private';

const path = `src/lib/rcon/dylib/rcon.${suffix}`;

const {
	symbols: { Execute }
} = dlopen(path, {
	Execute: {
		args: [FFIType.cstring, FFIType.cstring, FFIType.cstring],
		returns: FFIType.cstring
	}
});

const addr = `${PRIVATE_RCON_HOST}:${PRIVATE_RCON_PORT}`;
const password = PRIVATE_RCON_PASSWORD;

export const execute = (cmd: string) =>
	Execute(Buffer.from(addr), Buffer.from(password), Buffer.from(cmd)).toString();
