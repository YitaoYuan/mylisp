# rust实践报告

## 1 简介

本次实践使用rust实现了一个lisp解释器，它能接收的语言与标准的lisp有所不同，是一个个性化的lisp方言，具体表现在:(1)支持了数组，可以下标访问，可以轻易嵌套，可变长，并且数组元素类型可变。(2)丰富而简洁的运算符支持。(3)类似python的传引用风格。

代码位于[YitaoYuan/mylisp (github.com)](https://github.com/YitaoYuan/mylisp)。

## 2 具体功能

### 2.1 运算符

支持C语言中的所有四则运算，所有比较运算，所有位运算，逻辑运算（暂未支持短路求值，因为我觉得`if`可以做这件事）。

### 2.2 函数和语法

支持函数调用。

支持了lisp中的常见内置语法，如define，if，begin，list。

额外实现了方言化的内置函数，如append，printint，printchar，readint，

特别的，为了简化赋值和初始化，内置的赋值函数被设计为`(= name var)`的形式，并且支持不声明直接赋值。

### 2.3 数组

本项目拓展了原list的含义，现在它可以直接使用下标进行访问和赋值。

具体来讲`([] a i)`被用来访问`a[i]`，`([]= a i var)`被用来赋值，等价于`a[i] = var`。

得益于类python的传引用实现，数组的元素类型是不确定的，并且能轻易的嵌套，这一点类似python的list。

## 3 实现

实现上使用了rust的大量语法特性，包括但不限于Rc，RefCell，trait，enum，函数闭包，模式匹配，切片等等。

使用了expect，panic!，assert!等函数进行了相当多的错误处理。

数据结构上使用了HashMap，Vec和String等等。

## 4 示例

我编写了两个lisp程序来验证模型。两份lisp代码大小分别93行和75行，位于data文件夹下。

### 4.1 输出所有阶乘

在这个样例里，我编写的lisp代码使用了经典的next permutation算法。C++ STL库中也使用了此算法，时间复杂度均摊O(1)。

运行命令：

```
.../mylisp> cargo run data/permutation.lisp
```

输入：

```
4
```

输出：

```
1 2 3 4
1 2 4 3
1 3 2 4
1 3 4 2
1 4 2 3
1 4 3 2
2 1 3 4
2 1 4 3
2 3 1 4
2 3 4 1
2 4 1 3
2 4 3 1
3 1 2 4
3 1 4 2
3 2 1 4
3 2 4 1
3 4 1 2
3 4 2 1
4 1 2 3
4 1 3 2
4 2 1 3
4 2 3 1
4 3 1 2
4 3 2 1
```

### 4.2 矩阵乘法

这里演示2*2的矩阵乘法，预先设定矩阵A和B的值，结果保存在矩阵C中。

运行命令：

```
.../mylisp> cargo run data/matrix_multiply.lisp
```

输入：无

输出：

```
A
1 2
3 4
B
1 1
2 4
C
5 9
11 19
```

## 附录

![1](C:\Users\86133\Desktop\homework\6\rust\mylisp\1.png)

![2](C:\Users\86133\Desktop\homework\6\rust\mylisp\2.png)