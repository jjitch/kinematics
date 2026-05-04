import { describe, it, expect, vi } from "vitest";
import { UndoStack } from "./undo.js";
import type { Command } from "./undo.js";

function makeCmd(label: string, log: string[]): Command {
  return {
    description: label,
    execute: () => log.push(`exec:${label}`),
    undo: () => log.push(`undo:${label}`),
  };
}

describe("UndoStack", () => {
  it("executes command immediately on push", () => {
    const log: string[] = [];
    const stack = new UndoStack();
    stack.push(makeCmd("A", log));
    expect(log).toEqual(["exec:A"]);
  });

  it("canUndo after push, canRedo starts false", () => {
    const stack = new UndoStack();
    expect(stack.canUndo()).toBe(false);
    stack.push(makeCmd("A", []));
    expect(stack.canUndo()).toBe(true);
    expect(stack.canRedo()).toBe(false);
  });

  it("undo calls command undo and decrements cursor", () => {
    const log: string[] = [];
    const stack = new UndoStack();
    stack.push(makeCmd("A", log));
    log.length = 0;
    stack.undo();
    expect(log).toEqual(["undo:A"]);
    expect(stack.canUndo()).toBe(false);
    expect(stack.canRedo()).toBe(true);
  });

  it("redo re-executes command", () => {
    const log: string[] = [];
    const stack = new UndoStack();
    stack.push(makeCmd("A", log));
    stack.undo();
    log.length = 0;
    stack.redo();
    expect(log).toEqual(["exec:A"]);
    expect(stack.canRedo()).toBe(false);
  });

  it("push discards redo history", () => {
    const log: string[] = [];
    const stack = new UndoStack();
    stack.push(makeCmd("A", log));
    stack.push(makeCmd("B", log));
    stack.undo();
    stack.push(makeCmd("C", log));
    expect(stack.canRedo()).toBe(false);
    log.length = 0;
    stack.undo();
    expect(log).toEqual(["undo:C"]);
  });

  it("respects max depth by dropping oldest command", () => {
    const stack = new UndoStack(3);
    const log: string[] = [];
    stack.push(makeCmd("A", log));
    stack.push(makeCmd("B", log));
    stack.push(makeCmd("C", log));
    stack.push(makeCmd("D", log)); // should evict A
    log.length = 0;
    // Undo three times: should get D, C, B — not A
    stack.undo(); // undo D
    stack.undo(); // undo C
    stack.undo(); // undo B
    expect(stack.canUndo()).toBe(false);
    expect(log.map(s => s.split(":")[1])).toEqual(["D", "C", "B"]);
  });

  it("undo returns false when nothing to undo", () => {
    const stack = new UndoStack();
    expect(stack.undo()).toBe(false);
  });

  it("redo returns false when nothing to redo", () => {
    const stack = new UndoStack();
    expect(stack.redo()).toBe(false);
  });

  it("onStateChange fires on push, undo, redo, clear", () => {
    const stack = new UndoStack();
    const cb = vi.fn();
    stack.onStateChange(cb);
    stack.push(makeCmd("A", []));
    stack.undo();
    stack.redo();
    stack.clear();
    expect(cb).toHaveBeenCalledTimes(4);
  });

  it("clear resets all state", () => {
    const stack = new UndoStack();
    stack.push(makeCmd("A", []));
    stack.clear();
    expect(stack.canUndo()).toBe(false);
    expect(stack.canRedo()).toBe(false);
  });
});
