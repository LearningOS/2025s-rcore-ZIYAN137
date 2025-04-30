# 编程作业
## 死锁检测
mutex只要发现无法申请加锁就判断为死锁即可
semaphore的话
每个线程内部自己维护 `need` 和 `allocation` 数组，其中存放的是 `(sem_id, count)`。
然后在 `down` 的时候在从各个线程中汇总起来，进行死锁检测计算。

# 简答作业
## Q1.在我们的多线程实现中，当主线程 (即 0 号线程) 退出时，视为整个进程退出， 此时需要结束该进程管理的所有线程并回收其资源。
- 需要回收的资源有哪些？
  - 线程资源
  - 内存资源
  - 同步资源（如锁、信号量
  - IO资源
- 其他线程的 TaskControlBlock 可能在哪些位置被引用，分别是否需要回收，为什么？
  - 任务队列：需要回收，避免内存泄漏
  - 线程调度器：需要回收，调度器会持有无用的引用，这会导致内存泄漏或资源浪费。
  - 等待队列：线程在等待某个资源，但是线程已经退出了，如果没及时移除，可能会导致内存泄漏。


## Q2.对比以下两种 Mutex 中的实现，二者有什么区别？这些区别可能会导致什么问题？
应该是对locked状态的处理不同
Mutex1：无论是否有等待的任务，都会先将 locked 设为 false，然后再尝试唤醒等待队列中的任务。
Mutex2：只有在没有等待任务时，才将 locked 设为 false；如果有等待任务，locked 状态保持为 true。

在 Mutex1 中，当有等待任务时，将 locked 设为 false 后，可能会导致其他线程在等待的任务被唤醒并获取锁之前获得锁，造成竞争条件，破坏互斥锁的正确性。
Mutex2 的实现避免了这个问题，因为当有等待任务时，locked 保持为 true，确保了只有被唤醒的任务才能继续持有锁，维护了互斥性。


```rust
impl Mutex for Mutex1 {
   fn unlock(&self) {
      let mut mutex_inner = self.inner.exclusive_access();
      assert!(mutex_inner.locked);
      mutex_inner.locked = false;
      if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
         add_task(waking_task);
      }
   }
}

impl Mutex for Mutex2 {
   fn unlock(&self) {
      let mut mutex_inner = self.inner.exclusive_access();
      assert!(mutex_inner.locked);
      if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
         add_task(waking_task);
      } else {
         mutex_inner.locked = false;
      }
   }
}
```

# 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。