pub struct RenderTargetStack {
    target_stack: Vec<RenderTarget>,
}

pub enum RenderTarget {
    // BaseQueue(LineTokenCollection),
    BreakableEntry(BreakableEntry),
}

/*
desired:
        [ BaseQueue ]
        [ BaseQueue, BreakableEntry ]
        [ BaseQueue, BreakableEntry, BreakableEntry ]

        [ BaseQueue, T, T ] *
        [ BaseQueue, S, T ]


states:
        "empty" i.e. only a base queue
        "not emmpty" -- can pop a target (not the base queue)


allowed but illegal:
        []
        [ BaseQueue, BaseQueue ]
        [ BreakableEntry, BaseQueue ]
*/

struct BottomStack {
    base_queue: LineTokenCollection,
}

impl BottomStack {
    fn push_token(&mut Stack, token: Token);
    fn push_new_target(&mut Stack, target: RenderTarget) -> StackWithTarget;
}

struct StackWithTarget {
    base_queue: LineTokenCollection,
    // maybe: turn these fields into their own type
    top: RenderTarget,
    others: Vec<RenderTarget>,
}

impl StackWithTarget {
    fn pop(self) -> (RenderTarget, TargetStack) {

        let next_stack = if let Some(target) = self.others.pop() {
            Target::StackWithTarget(StackWithTarget {
                base_queue: self.base_queue,
                top: target,
                others: self.others,
            })
        } else {
            TargetStack::Empty(BottomStack { base_queue: self.base_queue })
        };

        (self.top, next_stack)
    }
}

struct MutableStack;

enum TargetStack {
    Empty(BottomStack),
    StackWithTarget(StackWithTarget),
}

impl TargetStack {
    // fn push_token(&mut self, token: Token);

    fn push_new_target(&mut self, target: RenderTarget) -> StackWithTarget {
        match self {
            Target::Empty(bottom_stack) => {
                let mut stack_with_target = StackWithTarget {
                    base_queue: /* empty LineTokenCollection */,
                    top: target,
                    others: vec![],
                };

                std::mem::swap(&mut self.base_queue, &mut stack_with_target.base_queue);

                stack_with_target
            },
            Target::StackWithTarget(stack_with_target) => { /* todo */ }
        }
    }

    fn with_new_target<F>(&mut self, target: RenderTarget, f: F)
    where
        F: FnOnce(&mut StackWithTarget)
    {
        use TargetStack::*;

        let stack = self.push_new_target(target); // StackWithTarget
        f(&mut stack);
        let (target, next_stack) = stack.pop();

        match (self, next_stack) {
            (Empty(left), Empty(right)) => {
                std::mem::swap(&mut left.base_queue, &mut right.base_queue);
            },
            (Empty(left), StackWithTarget(right)) => {
                // push to empty stack
                std::mem::swap(&mut left.base_queue, &mut right.base_queue);
            },
            (StackWithTarget(left), Empty(right)) => {
                // impossible
                panic!();
            },
            (StackWithTarget(left), StackWithTarget(right)) => {
                // push to not-empty stack
                std::mem::swap(&mut left.base_queue, &mut right.base_queue);
            },
        }
    }
}
