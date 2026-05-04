export interface Keyframe {
  time: number;
  jointValues: Record<number, number>;
}

export class Timeline {
  private keyframes: Keyframe[] = [];
  private _currentTime = 0;
  private _duration = 5.0;
  private _playing = false;
  private _looping = false;
  private startWallTime = 0;
  private frameCb: ((t: number, vals: Record<number, number>) => void) | null = null;
  private animId = 0;

  recordKeyframe(time: number, jointValues: Record<number, number>): void {
    const idx = this.keyframes.findIndex((k) => Math.abs(k.time - time) < 0.01);
    const kf: Keyframe = { time, jointValues: { ...jointValues } };
    if (idx >= 0) {
      this.keyframes[idx] = kf;
    } else {
      this.keyframes.push(kf);
      this.keyframes.sort((a, b) => a.time - b.time);
    }
  }

  clearKeyframes(): void {
    this.keyframes = [];
  }

  getKeyframes(): readonly Keyframe[] {
    return this.keyframes;
  }

  setDuration(d: number): void {
    this._duration = d;
  }

  getDuration(): number {
    return this._duration;
  }

  getCurrentTime(): number {
    return this._currentTime;
  }

  setTime(t: number): void {
    this._currentTime = Math.max(0, Math.min(t, this._duration));
    const vals = this.interpolate(this._currentTime);
    if (vals) this.frameCb?.(this._currentTime, vals);
  }

  play(): void {
    if (this._playing) return;
    this._playing = true;
    this.startWallTime = performance.now() - this._currentTime * 1000;
    const tick = (): void => {
      if (!this._playing) return;
      const elapsed = (performance.now() - this.startWallTime) / 1000;
      if (elapsed >= this._duration) {
        if (this._looping) {
          this.startWallTime = performance.now();
          this._currentTime = 0;
        } else {
          this._currentTime = this._duration;
          this._playing = false;
          const vals = this.interpolate(this._currentTime);
          if (vals) this.frameCb?.(this._currentTime, vals);
          return;
        }
      } else {
        this._currentTime = elapsed;
      }
      const vals = this.interpolate(this._currentTime);
      if (vals) this.frameCb?.(this._currentTime, vals);
      this.animId = requestAnimationFrame(tick);
    };
    this.animId = requestAnimationFrame(tick);
  }

  pause(): void {
    this._playing = false;
    cancelAnimationFrame(this.animId);
  }

  stop(): void {
    this.pause();
    this.setTime(0);
  }

  isPlaying(): boolean {
    return this._playing;
  }

  setLoop(v: boolean): void {
    this._looping = v;
  }

  onTimeUpdate(cb: (t: number, vals: Record<number, number>) => void): void {
    this.frameCb = cb;
  }

  private interpolate(t: number): Record<number, number> | null {
    if (this.keyframes.length === 0) return null;
    if (this.keyframes.length === 1) return { ...this.keyframes[0].jointValues };

    let before = this.keyframes[0];
    let after = this.keyframes[this.keyframes.length - 1];
    for (let i = 0; i < this.keyframes.length - 1; i++) {
      if (this.keyframes[i].time <= t && this.keyframes[i + 1].time >= t) {
        before = this.keyframes[i];
        after = this.keyframes[i + 1];
        break;
      }
    }

    if (before === after) return { ...before.jointValues };

    const dt = after.time - before.time;
    const alpha = dt > 0 ? (t - before.time) / dt : 0;
    const result: Record<number, number> = {};
    const ids = new Set([
      ...Object.keys(before.jointValues).map(Number),
      ...Object.keys(after.jointValues).map(Number),
    ]);
    for (const id of ids) {
      const v0 = before.jointValues[id] ?? 0;
      const v1 = after.jointValues[id] ?? 0;
      result[id] = v0 + (v1 - v0) * alpha;
    }
    return result;
  }
}
