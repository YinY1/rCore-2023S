# lab4 report

## 过程

花了很长时间去理解文件系统的具体操作，linkat部分文档说的有点奇怪（只是我理解能力不行），还得看测例的注释和过程来判断old_name和new_name到底是哪个link哪个

unlink投机了，没有删掉无用的dirent，直接用empty代替了原来的位置。不过参考答案给的也只是笨方法，很吃效率故懒得实现了。

## 结果

我不知道为什么参考答案用的spawn = fork + exec 能过测例，但是按照ch5的spawn简单修改后却会在重复运行之后报错 `Instuction Pagefault bad addr = 0x0`，其他的全pass