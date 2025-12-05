typedef struct {
    int* stack;
    int top;
    int capacity;
} Stack;

Stack* createStack(int capacity) {
    Stack* stack = (Stack*)malloc(sizeof(Stack));
    stack->capacity = capacity;
    stack->top = -1;
    stack->stack = (int*)malloc(capacity * sizeof(int));
    return stack;
}

void pushStack(Stack* stack, int x) {
    stack->stack[++stack->top] = x;
}

int popStack(Stack* stack) {
    return stack->stack[stack->top--];
}

int topStack(Stack* stack) {
    return stack->stack[stack->top];
}

int isEmptyStack(Stack* stack) {
    return stack->top == -1;
}

typedef struct {
    Stack* input;
    Stack* output;
} MyQueue;

// Forward declaration
int myQueuePeek(MyQueue* obj);

MyQueue* myQueueCreate() {
    MyQueue* queue = (MyQueue*)malloc(sizeof(MyQueue));
    queue->input = createStack(100);
    queue->output = createStack(100);
    return queue;
}

void myQueuePush(MyQueue* obj, int x) {
    pushStack(obj->input, x);
}

int myQueuePop(MyQueue* obj) {
    int val = myQueuePeek(obj);
    popStack(obj->output);
    return val;
}

int myQueuePeek(MyQueue* obj) {
    if (isEmptyStack(obj->output)) {
        while (!isEmptyStack(obj->input)) {
            pushStack(obj->output, popStack(obj->input));
        }
    }
    return topStack(obj->output);
}

bool myQueueEmpty(MyQueue* obj) {
    return isEmptyStack(obj->input) && isEmptyStack(obj->output);
}

void myQueueFree(MyQueue* obj) {
    free(obj->input->stack);
    free(obj->input);
    free(obj->output->stack);
    free(obj->output);
    free(obj);
}