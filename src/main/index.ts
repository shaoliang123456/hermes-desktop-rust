const native = require("../../native/hermes-native.darwin-arm64.node");

import { app, BrowserWindow, ipcMain } from "electron";
import { join } from "path";

let mainWindow: BrowserWindow | null = null;

function createWindow(): void {
  mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    webPreferences: {
      preload: join(__dirname, "../preload/index.js"),
      sandbox: false,
    },
  });
  if (process.env.ELECTRON_RENDERER_URL) {
    mainWindow.loadURL(process.env.ELECTRON_RENDERER_URL);
  } else {
    mainWindow.loadFile(join(__dirname, "../renderer/index.html"));
  }
}

app.whenReady().then(() => {
  registerIpcHandlers();
  createWindow();
  app.on("activate", () => { if (BrowserWindow.getAllWindows().length === 0) createWindow(); });
});
app.on("window-all-closed", () => { if (process.platform !== "darwin") app.quit(); });

function registerIpcHandlers(): void {
  ipcMain.handle("list-cached-sessions", (_e, limit?: number) => native.listCachedSessions(limit));
  ipcMain.handle("sync-session-cache", (_e, profile?: string) => native.syncSessionCache(profile));
  ipcMain.handle("delete-session", (_e, id: string) => native.deleteSession(id));
  ipcMain.handle("update-session-title", (_e, id: string, title: string) => native.updateSessionTitle(id, title));
  ipcMain.handle("search-sessions", (_e, q: string) => native.searchSessions(q));
  ipcMain.handle("kanban-list-boards", (_e, archived?: boolean, profile?: string) => native.kanbanListBoards(archived, profile));
  ipcMain.handle("kanban-list-tasks", (_e, board?: string, archived?: boolean, profile?: string) => native.kanbanListTasks(board, archived, profile));
  ipcMain.handle("kanban-dispatch", (_e, board?: string, profile?: string) => native.kanbanDispatch(board, profile));
  ipcMain.handle("get-config", () => native.getConfig());
  ipcMain.handle("save-config", (_e, config: unknown) => native.saveConfig(config));
  ipcMain.handle("check-hermes-installation", () => native.checkHermesInstallation());
  ipcMain.handle("list-profiles", (_e, active?: string) => native.listProfiles(active));
  ipcMain.handle("list-models-cached", () => native.listModelsCached());
  ipcMain.handle("ssh-tunnel-status", () => native.sshTunnelStatus());
  ipcMain.handle("parse-sse-chunk", (_e, chunk: Buffer) => native.parseSseChunk(chunk));
}
