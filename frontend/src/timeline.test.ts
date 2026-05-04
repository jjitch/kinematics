import { describe, it, expect, vi } from "vitest";
import { Timeline } from "./timeline.js";

describe("Timeline", () => {
  it("starts with no keyframes", () => {
    const tl = new Timeline();
    expect(tl.getKeyframes()).toHaveLength(0);
  });

  it("recordKeyframe adds a keyframe", () => {
    const tl = new Timeline();
    tl.recordKeyframe(0, { 0: 0.5 });
    expect(tl.getKeyframes()).toHaveLength(1);
    expect(tl.getKeyframes()[0].time).toBe(0);
    expect(tl.getKeyframes()[0].jointValues[0]).toBe(0.5);
  });

  it("recordKeyframe sorts by time", () => {
    const tl = new Timeline();
    tl.recordKeyframe(2, { 0: 0.5 });
    tl.recordKeyframe(0, { 0: 0.1 });
    tl.recordKeyframe(1, { 0: 0.3 });
    const times = tl.getKeyframes().map(k => k.time);
    expect(times).toEqual([0, 1, 2]);
  });

  it("recordKeyframe replaces existing keyframe at same time", () => {
    const tl = new Timeline();
    tl.recordKeyframe(1, { 0: 0.5 });
    tl.recordKeyframe(1, { 0: 0.9 });
    expect(tl.getKeyframes()).toHaveLength(1);
    expect(tl.getKeyframes()[0].jointValues[0]).toBe(0.9);
  });

  it("clearKeyframes empties the list", () => {
    const tl = new Timeline();
    tl.recordKeyframe(0, { 0: 0.5 });
    tl.clearKeyframes();
    expect(tl.getKeyframes()).toHaveLength(0);
  });

  it("interpolate returns single keyframe value at any time", () => {
    const tl = new Timeline();
    tl.recordKeyframe(0, { 0: 1.0 });
    const cb = vi.fn();
    tl.onTimeUpdate(cb);
    tl.setTime(2.5);
    expect(cb).toHaveBeenCalledWith(2.5, expect.objectContaining({ 0: 1.0 }));
  });

  it("interpolate linearly between two keyframes", () => {
    const tl = new Timeline();
    tl.recordKeyframe(0, { 0: 0.0 });
    tl.recordKeyframe(2, { 0: 2.0 });
    const cb = vi.fn();
    tl.onTimeUpdate(cb);
    tl.setTime(1);
    const vals = cb.mock.calls[0][1] as Record<number, number>;
    expect(vals[0]).toBeCloseTo(1.0);
  });

  it("setTime clamps to [0, duration]", () => {
    const tl = new Timeline();
    tl.setDuration(5);
    tl.recordKeyframe(0, { 0: 0.0 });
    tl.setTime(-1);
    expect(tl.getCurrentTime()).toBe(0);
    tl.setTime(10);
    expect(tl.getCurrentTime()).toBe(5);
  });

  it("isPlaying starts false", () => {
    expect(new Timeline().isPlaying()).toBe(false);
  });

  it("pause stops playback", () => {
    const tl = new Timeline();
    tl.recordKeyframe(0, { 0: 0 });
    tl.recordKeyframe(5, { 0: 1 });
    tl.play();
    expect(tl.isPlaying()).toBe(true);
    tl.pause();
    expect(tl.isPlaying()).toBe(false);
  });

  it("stop resets time to 0", () => {
    const tl = new Timeline();
    tl.recordKeyframe(0, { 0: 0 });
    tl.recordKeyframe(5, { 0: 1 });
    tl.play();
    tl.stop();
    expect(tl.isPlaying()).toBe(false);
    expect(tl.getCurrentTime()).toBe(0);
  });
});
