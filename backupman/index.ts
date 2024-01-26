import {
  S3Client,
  ListObjectsCommand,
  PutObjectCommand,
  DeleteObjectCommand,
  type _Object,
} from "@aws-sdk/client-s3";
import path from "path";
import { $ } from "bun";
import { parseArgs } from "util";

const { values } = parseArgs({
  args: Bun.argv,
  options: {
    access_key: {
      type: "string",
      default: process.env.LINODE_S3_ACCESS_KEY,
    },

    secret_key: {
      type: "string",
      default: process.env.LINODE_S3_SECRET_KEY,
    },

    region: {
      type: "string",
      default: process.env.LINODE_S3_REGION,
    },

    endpoint: {
      type: "string",
      default: process.env.LINODE_S3_ENDPOINT,
    },

    bucket: {
      type: "string",
      default: process.env.LINODE_S3_BUCKET,
    },

    save_games_path: {
      type: "string",
      default: process.env.PALWORLD_SAVE_GAMES_PATH,
    },

    backup_retention_count: {
      type: "string",
      default: process.env.BACKUP_RETENTION_COUNT,
    },
  },
  strict: true,
  allowPositionals: true,
});

const requiredValues = [
  "access_key",
  "secret_key",
  "region",
  "endpoint",
  "bucket",
  "save_games_path",
  "backup_retention_count",
];
for (const value of requiredValues) {
  if (
    !values[value as keyof typeof values] ||
    values[value as keyof typeof values] === ""
  ) {
    console.error(`Missing required value: ${value}`);
    process.exit(1);
  }
}

const options = values as Record<
  keyof typeof values,
  NonNullable<(typeof values)[keyof typeof values]>
>;

const BACKUP_RETENTION_COUNT = parseInt(options.backup_retention_count);

const client = new S3Client({
  region: options.region,
  endpoint: options.endpoint,
  credentials: {
    accessKeyId: options.access_key,
    secretAccessKey: options.secret_key,
  },
});

const newBackup = await createBackup();
console.log(
  `Successfully created backup: ${newBackup} at ${new Date().toUTCString()}`
);

const trimmedBackups = await trimBackups();
if (trimmedBackups.length) {
  console.log(
    `Successfully trimmed ${
      trimmedBackups.length
    } backups:\n${trimmedBackups.join(",\n")}\n at ${new Date().toUTCString()}`
  );
} else {
  console.log(`No backups to trim at ${new Date().toUTCString()}`);
}

export async function createBackup(): Promise<string> {
  try {
    const backupName = `pw-backup-${Date.now()}.zip`;
    const backupPath = `/tmp/${backupName}`;

    const file = Bun.file(backupPath);

    await $`cd ${path.join(
      options.save_games_path,
      ".."
    )} && zip -r ${backupPath} ${getLastDirectoryName(
      options.save_games_path
    )}/`;

    const cmd = new PutObjectCommand({
      Bucket: options.bucket,
      Key: backupName,
      Body: Buffer.from(await file.arrayBuffer()),
    });

    await client.send(cmd);

    return backupName;
  } catch (e) {
    console.error("Failed to create backup", e);
    process.exit(1);
  }
}

export async function listBackups(): Promise<_Object[]> {
  try {
    const command = new ListObjectsCommand({
      Bucket: options.bucket,
    });

    const data = await client.send(command);

    return data.Contents || [];
  } catch (e) {
    console.error("Failed to list backups", e);
    process.exit(1);
  }
}

export async function deleteBackup(backupName: string) {
  try {
    const command = new DeleteObjectCommand({
      Bucket: options.bucket,
      Key: backupName,
    });

    await client.send(command);
  } catch (e) {
    console.error("Failed to delete backup", e);
    process.exit(1);
  }
}

export async function trimBackups(): Promise<string[]> {
  try {
    let backups = await listBackups();
    if (backups.length <= BACKUP_RETENTION_COUNT) {
      return [];
    }

    backups = sortBackups(backups);

    const backupsToDelete = backups.slice(BACKUP_RETENTION_COUNT);
    for (const backup of backupsToDelete) {
      await deleteBackup(backup.Key!);
    }

    return backupsToDelete.map((backup) => backup.Key!);
  } catch (e) {
    console.error("Failed to trim backups", e);
    process.exit(1);
  }
}

// sorts backups from newest to oldest
function sortBackups(backups: _Object[]): _Object[] {
  return backups.sort((a, b) => {
    return (
      getTimestampFromBackupName(b.Key!).getTime() -
      getTimestampFromBackupName(a.Key!).getTime()
    );
  });
}

function getTimestampFromBackupName(name: string): Date {
  const timestamp = name.split("-")[2].split(".")[0];
  return new Date(parseInt(timestamp));
}

export function getLastDirectoryName(fullPath: string): string {
  const pathWithSep = fullPath.endsWith(path.sep)
    ? fullPath
    : fullPath + path.sep;

  return path.basename(pathWithSep);
}
