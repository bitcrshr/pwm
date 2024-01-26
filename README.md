# PalWorld Manager

As of right now, this is just a hodgepodge monorepo of tools I'm using to manage my PalWorld server, and are mostly suited to my needs/not generalized.

A quick overview of the dirs:

- backupman
  - A small Bun script used to back up the save files to an S3-compatible bucket (I use Linode). Configurable to keep a certain number of recent backups. Intended to be used with crontab or similar. I made this to save $5/mo on server backups AND have more control/granularity of them :)
- gorcon-ffi
  - A tiny wrapper around [gorcon/rcon](https://github.com/gorcon/rcon) that's very simple and just compiles to a cdylib to be consumed by Bun. At the time of writing, gorcon is the only stable RCON client I found that works with PalWorld (and I'm too lazy write my own in a diff lang).
- web-ssr
  - A SvelteKit/Bun project that will act as the web interface to be able to see active players and do server maintenance/admin stuff. Interfaces with RCON on the backend.
  - Be smart about where you put this and who has access to it, recommend very tight auth, or better yet, don't host it publicly and use VPN/TailScale. It's nowhere near ready to be out in the open, and it's probably not the best move to expose an RCON interface publicly.
