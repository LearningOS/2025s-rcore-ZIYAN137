# 编程作业

首先在 `config.rs` 中添加常量 `MAX_SYSCALL_NUM = 500`

然后修改 `TaskControlBlock` 的结构，添加了 `syscall-times` ，用于记录task对于每个系统调用的次数。
``` Rust
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// syscall times
    pub syscall_times: [u32; MAX_SYSCALL_NUM]
}
```

然后 `task` 模块中添加相关的计数和获取的相关函数
``` Rust
fn cnt_syscall(&self, syscall_id: usize) {
    ...
}

fn get_syscall_times(&self) -> [u32; MAX_SYSCALL_NUM] {
    ...
}
```

在syscall中添加一行，每次syscall时触发计数。
``` Rust
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    ...
    cnt_syscall(syscall_id);    
    ...
}
```

最后实现 `sys_trace` 即可

# 简答作业
## Q1
正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

## A1
ch2b_bad_address.rs
程序尝试向地址 0x0 写入数据，触发地址访问异常（Address Fault）。
错误行为：程序陷入异常处理流程，打印错误信息或直接终止。

ch2b_bad_instructions.rs
程序尝试执行 sret 指令（S 态特权指令），触发非法指令异常（Illegal Instruction）。
错误行为：程序无法继续执行，陷入异常处理流程。

ch2b_bad_register.rs
程序尝试读取 sstatus 寄存器（S 态特权寄存器），触发非法指令异常（Illegal Instruction）。
错误行为：程序无法继续执行，陷入异常处理流程。

## Q2
深入理解 trap.S 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下问题:

### Q2.1
L40：刚进入 `__restore` 时，`sp` 代表了什么值。请指出 `__restore` 的两种使用情景。

### A2.1
 `sp` 指向内核栈中保存的TrapContext的起始地址。使用场景：

从 S 态返回到 U 态，恢复用户态的寄存器和状态。

从中断或异常处理返回到用户态，恢复用户态的执行上下文。

### Q2.2
L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。
``` asm
ld t0, 32*8(sp)
ld t1, 33*8(sp)
ld t2, 2*8(sp)
csrw sstatus, t0
csrw sepc, t1
csrw sscratch, t2
```

### A2.2 

特殊处理了 `sstatus` , `sepc`, `sscratch` 这三个寄存器。
* `sstatus`: 一个状态寄存器，包含了处理器的当前状态信息,恢复sstatus确保返回用户态时具有正确状态
* `sepc`: 保存了处理器需要返回的指令地址，恢复sepc使得处理器能够在处理完异常或中断后继续执行被中断的程序
* `sscratch`: 临时寄存器，此处保存了用户栈指针，临时存放，使得处理器能够在处理完异常或中断后继续执行被中断的程序

### Q2.3
L50-L56：为何跳过了 `x2` 和 `x4`？
``` asm
ld x1, 1*8(sp)
ld x3, 3*8(sp)
.set n, 5
.rept 27
   LOAD_GP %n
   .set n, n+1
.endr
```

### A2.3

x2应该是2*8(sp)，但是这里是栈指针，此时已经存在sscratch中。x4是tp线程指针寄存器，用户态程序不使用

### Q2.4
L60：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？
``` asm
csrrw sp, sscratch, sp
```

### A2.4
sscratch是内核栈指针， sp是用户栈指针

### Q2.5
__restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

### A2.5
`sret` 返回原来的程序流

### Q2.6
L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？
``` asm
csrrw sp, sscratch, sp
```

### A2.6
`sp` 切换为内核栈指针，`sscratch` 保存用户态指针

### Q2.7
从 U 态进入 S 态是哪一条指令发生的？

### A2.7
触发trap后，通过L13切换到内核栈，从 U 态进入 S 态

# 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

    无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。