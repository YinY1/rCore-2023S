# lab3 report

## 过程

简单用了优先队列实现调度。就是rust用小顶堆要么用`reverse`多套一层，要么反直觉定义一下`partialEq`把`LESS`和`GREATER`反着来。

~~就不能像c++一样传一个`std::greater`进去吗~~

## 结果

- [x] 必做
- [x] MLFQ
- [ ] EDF (要弄成实时系统感觉不太好弄)
- [ ] COW