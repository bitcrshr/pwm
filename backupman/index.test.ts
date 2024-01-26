import { expect, test, beforeAll, afterAll } from "bun:test";
import {
  createBackup,
  deleteBackup,
  getLastDirectoryName,
  listBackups,
  trimBackups,
} from "./index";

let createdObjects: string[] = [];

afterAll(async () => {
  for (const object of createdObjects) {
    await deleteBackup(object);
  }
});

test("can create a backup", async () => {
  const backupName = await createBackup();
  createdObjects.push(backupName);

  expect(backupName).toBeString();
  expect(backupName).not.toBeEmpty();
});

test("can get the right dir name", () => {
  const path = "/home/steam/palworld/savegames";

  expect(getLastDirectoryName(path)).toBe("savegames");
});

test("can delete a backup", async () => {
  const backupName = await createBackup();
  createdObjects.push(backupName);

  expect(deleteBackup(backupName)).resolves.toBeUndefined();
  createdObjects.pop();
});

test("can list backups", async () => {
  const backupName = await createBackup();
  createdObjects.push(backupName);

  const backups = await listBackups();
  expect(backups).toBeArray();
  expect(backups).not.toBeEmpty();
});

test("can trim backups", async () => {
  let oldBackups: string[] = [];
  for (let i = 0; i < 5; i++) {
    const backupName = await createBackup();
    createdObjects.push(backupName);
    oldBackups.push(backupName);
  }

  for (let i = 0; i < parseInt(process.env.BACKUP_RETENTION_COUNT!); i++) {
    const backupName = await createBackup();
    createdObjects.push(backupName);
  }

  await trimBackups();

  const backups = await listBackups();

  expect(backups).toBeArrayOfSize(10);
  for (const backup of oldBackups) {
    expect(backups).not.toContain(backup);
  }
});
