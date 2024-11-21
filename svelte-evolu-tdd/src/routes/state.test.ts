import { describe, test, expect, beforeEach } from "vitest";
import { createActor } from "xstate";
import { machine, State, type Context } from "./state";

describe("Todo Machine", () => {
  let actor: ReturnType<typeof createActor>;
  let testTask: Context.Task;

  beforeEach(() => {
    // Create a fresh actor instance before each test
    actor = createActor(machine);
    actor.start();

    // Setup test task
    testTask = {
      id: "1",
      name: "Test Task",
      done: false,
    };
  });

  test("should start in Idling state", () => {
    expect(actor.getSnapshot().matches(State.Idling)).toBe(true);
  });

  test("should have empty tasksMap initially when no tasks are added", () => {
    expect(actor.getSnapshot().context.tasksMap.size).toBe(0);
  });

  describe("Add event", () => {
    test("should add task to tasksMap", () => {
      // Send Add event
      actor.send({ type: "Add", id: testTask.id, name: testTask.name });

      expect(actor.getSnapshot().context.tasksMap.has(testTask.id)).toBe(true);
    });
  });

  describe("Check event", () => {
    beforeEach(() => {
      actor.send({ type: "Add", id: testTask.id, name: testTask.name });
    })

    test("should mark task as done", () => {
      actor.send({ type: "Check", id: testTask.id });
      const updatedTask = actor.getSnapshot().context.tasksMap.get(testTask.id);

      expect(updatedTask?.done).toBe(true);
    });

    test("should not modify tasksMap if task id does not exist", () => {
      const beforeSize = actor.getSnapshot().context.tasksMap.size;
      actor.send({ type: "Check", id: "non-existent" });

      const afterSize = actor.getSnapshot().context.tasksMap.size;
      expect(afterSize).toBe(beforeSize);
    });
  });

  describe("Uncheck event", () => {
    beforeEach(() => {
      actor.send({ type: "Add", id: testTask.id, name: testTask.name });
    })

    test("should mark task as not done", () => {
      actor.send({ type: "Check", id: testTask.id });
      actor.send({ type: "Uncheck", id: testTask.id });

      const updatedTask = actor.getSnapshot().context.tasksMap.get(testTask.id);
      expect(updatedTask?.done).toBe(false);
    });

    test("should not modify tasksMap if task id does not exist", () => {
      const beforeSize = actor.getSnapshot().context.tasksMap.size;

      // Send Uncheck event with non-existent id
      actor.send({ type: "Uncheck", id: "non-existent" });

      // Verify tasksMap wasn't modified
      const afterSize = actor.getSnapshot().context.tasksMap.size;
      expect(afterSize).toBe(beforeSize);
    });
  });

  describe("Remove event", () => {
    beforeEach(() => {
      actor.send({ type: "Add", id: testTask.id, name: testTask.name });
    })

    test("should remove task from tasksMap", () => {
      actor.send({ type: "Remove", id: testTask.id });

      expect(actor.getSnapshot().context.tasksMap.has(testTask.id)).toBe(false);
    });

    test("should not modify tasksMap if task id does not exist", () => {
      const beforeSize = actor.getSnapshot().context.tasksMap.size;
      actor.send({ type: "Remove", id: "non-existent" });

      const afterSize = actor.getSnapshot().context.tasksMap.size;
      expect(afterSize).toBe(beforeSize);
    });
  });

  describe("Context mutations", () => {
    test("should create new Map references for task updates", () => {
      const beforeMap = actor.getSnapshot().context.tasksMap;
      actor.send({ type: "Check", id: testTask.id });

      const afterMap = actor.getSnapshot().context.tasksMap;
      expect(beforeMap === afterMap).toBe(true);

      const updatedTask = afterMap.get(testTask.id);
      expect(updatedTask).not.toBe(testTask);
    });
  });
});
