import { describe, expect, test, beforeAll, afterAll } from "bun:test";
import * as fs from "fs";

// Must be initialized before importing the TS file so it picks up the mock!
fs.writeFileSync(".screenpipe-permissions.json", JSON.stringify({
  pipe_name: "test-pipe",
  offline_mode: false,
  pipe_dir: "/tmp"
}));

// Use dynamic import so it evaluates AFTER the file is created
const plugin = (await import("./screenpipe-permissions.ts")).default;

describe("macOS TCC Sandbox", () => {
  let toolCallHandler: any = null;

  beforeAll(() => {
    const pi = {
      on: (event: string, handler: any) => {
        if (event === "tool_call") {
          toolCallHandler = handler;
        }
      }
    };
    plugin(pi as any);
  });

  afterAll(() => {
    fs.unlinkSync(".screenpipe-permissions.json");
  });

  test("initializes successfully", () => {
    expect(toolCallHandler).not.toBeNull();
  });

  test("blocks access to ~/Documents", async () => {
    const result = await toolCallHandler({ tool: "bash", input: { command: "ls ~/Documents" } });
    expect(result.block).toBe(true);
    expect(result.reason).toContain("Access to macOS protected folders");
  });

  test("blocks access to absolute /Users/.../Downloads", async () => {
    const result = await toolCallHandler({ tool: "bash", input: { command: "cat /Users/louis/Downloads/test.txt" } });
    expect(result.block).toBe(true);
  });

  test("allows access to unprotected folders like ~/Workspace", async () => {
    const result = await toolCallHandler({ tool: "bash", input: { command: "ls ~/Workspace" } });
    // Expect undefined or null since no block happened
    expect(result).toBeUndefined();
  });
});
