# 编程作业

## 1.迁移
直接迁移即可，只需稍微修改一下spawn，因为文件系统的引入，我们需要从文件系统中加载用户程序
``` Rust
pub fn sys_spawn(path: *const u8) -> isize {
    //...
    if let Some(app_inode) = open_file(path.as_str(), OpenFlags::RDONLY) {
        let data = app_inode.read_all();
        let new_task = current_task.spawn(data.as_slice());
        // ...
    }
    // ...
}
```

## 2.fstat
就是单纯的获得 `(ino, mode, nlink)` ，并把它们填到 `Stat` 之中
`nlink` 直接暴力遍历并计数即可
主要还是 `ino` 的获取， `Inode` 将自己的 `block_id` 和 `block_offset` 传给 `fs`
然后我们自己通过这两个参数算出 `ino`

## 3.linkat
仿照 `create()` 写了一个 `linkat`
根据 `old_name` 去读取 `old_inode_id` ，然后检测 `new_name` 是否存在，
写入一个新的 `new_name` ，其 `inode_id` 为 `old_inode_id`

## 4.unlinkat
直接暴力查找，如果找到对应的dirent，将其改为DirEntry::empty()

# 简答作业

## Q1.在我们的easy-fs中，root inode起着什么作用？如果root inode中的内容损坏了，会发生什么？
ROOT_INODE是根目录所对应的inode，如果ROOT_INODE损坏，整个文件系统也无法正确运行

## Q2.举出使用 pipe 的一个实际应用的例子。
一个命令的输出作为另一个命令的输入

## Q3.如果需要在多个进程间互相通信，则需要为每一对进程建立一个管道，非常繁琐，请设计一个更易用的多进程通信机制。
* 共享内存
* golang的channel

# 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。