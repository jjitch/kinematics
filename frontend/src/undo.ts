export interface Command {
  description: string;
  execute(): void;
  undo(): void;
}

export class UndoStack {
  private stack: Command[] = [];
  private cursor = -1;
  private maxDepth: number;
  private changeCb: (() => void) | null = null;

  constructor(maxDepth = 50) {
    this.maxDepth = maxDepth;
  }

  push(cmd: Command): void {
    this.stack.splice(this.cursor + 1);
    if (this.stack.length >= this.maxDepth) {
      this.stack.shift();
      this.cursor = this.stack.length - 1;
    }
    this.stack.push(cmd);
    this.cursor = this.stack.length - 1;
    cmd.execute();
    this.changeCb?.();
  }

  undo(): boolean {
    if (this.cursor < 0) return false;
    this.stack[this.cursor].undo();
    this.cursor--;
    this.changeCb?.();
    return true;
  }

  redo(): boolean {
    if (this.cursor >= this.stack.length - 1) return false;
    this.cursor++;
    this.stack[this.cursor].execute();
    this.changeCb?.();
    return true;
  }

  canUndo(): boolean {
    return this.cursor >= 0;
  }

  canRedo(): boolean {
    return this.cursor < this.stack.length - 1;
  }

  onStateChange(cb: () => void): void {
    this.changeCb = cb;
  }

  clear(): void {
    this.stack = [];
    this.cursor = -1;
    this.changeCb?.();
  }

  /** Snapshot-based command: applies new state, reverts to old on undo. */
  static chainCmd(
    description: string,
    prevJson: string,
    nextJson: string,
    apply: (json: string) => void,
  ): Command {
    return {
      description,
      execute: () => apply(nextJson),
      undo: () => apply(prevJson),
    };
  }
}
