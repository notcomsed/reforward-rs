/*MIT License

Copyright (c) 2024 notcomsed

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.*/

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
/*编译好后,用控制台输出更佳*/
/*build for reforward-rs*/
    let text = vec!["我宣布;\n",
	"rust不愧是最Worst的语言;\n",
	"..........;\n",
	"rust拥护者总是吹什么精准优雅的内存管理;\n",
	"..........;\n",
	"实际上本人的体验是就像饭没吃完就被抢了一样;\n",
	"..........;\n",
	"..........;\n",
	"常言道:;\n",
	"C/C++是自己做饭;\n",
	"吃完后需要自己打理餐桌(内存);\n",
	"..........;\n",
	"..........;\n",
	"java/go是下馆子;\n",
	"吃完后需要会有服务员(CG)帮你打理餐桌(内存);\n",
	"..........;\n",
	"而rust是去别人家吃饭;\n",
	"才吃一口，饭(内存)就被主人端走了;\n",
	"..........;\n",
	"端上来一个饭(变量);\n",
	"我才吃了一口(调用一次);\n",
	"紧接着准备再吃一口的时候;\n",
	"他告诉我，这个饭已经...;\n",
	"Have been removed???;\n",
	"..........;\n",
	"艹，老子饭才吃一口，还没吃完就给我端走了???;\n",
	"然后rustc建议我不要吃这炖饭;\n",
	"吃饭的时候再做一发一样的饭出来吃(value.clone());\n",
	"..........;\n",
	"???;\n",
	"好吧，谁叫在别人家吃饭呢;\n",
	"然后不断的做饭吃(clone())，不敢动没吃的饭(let mut)，以为这样就可以了;\n",
	"..........;\n",
	"结果吃到一半，他告诉我，value does not live long enough ???;\n",
	"用餐时间超时了，要把我的饭端走???;\n",
	"..........;\n",
	"rustc又建议我吃固定餐(static);\n",
	"然后固定餐端上来只能看不能吃???;\n",
	"..........;\n",
	"没法，只能用unsafe强行这个吃不能动的固定餐;\n",
	"折腾许久，总算是勉强吃完了这顿饭;\n",
	"..........;\n",
	"..........;\n",
	"既然rust邪教徒说，rust一旦编译完成后，基本没有错误，可以长久使用，有问题编译都不会通过\n",
	"那么我这个程序大概的确是没有问题的;\n",
	"如果有问题，肯定是邪教徒使用方法的问题doge;\n",
	"..........;\n",
	"..........;\n",
	"做为熟悉C/C++的人来说，rust确实不是很难;\n",
	"但是他不经过我同意直接回收我的内存,fu*k!,我还没用完呢;\n",
	"人怎么能被编译器欺负呢???;\n",
	"能让我低头的只有太阳！;\n",
	"..........;\n",
	"有人可能会说，使用unsafe管理内存啊;\n",
	"..........;\n",
	"我想说，;\n",
	"rust的内存安全就是依赖与rust编译器的管理的;\n",
	"如果用unsafe管理，那么和写C/C++内存管理又有什么区别呢;\n",
	"除了在不懂rust，又是rust的粉丝面前装B以外，没有其他作用;\n",
	"....;\n",
	"..........;\n",
	"rust就是如同arch一样邪教;\n",
	"一大群连rust都写不来的邪教徒，都到处鼓吹rust性能如何如何的强;\n",
	"有这时间，还不如写几个rust项目给我抄抄;\n",
	"..........;\n",
	"..........;\n",
	"rust的生态依赖库本来就少;\n",
	"来鼓吹让其他开发者用rust;\n",
	"..........;\n",
	"当其他开发者被忽悠，写rust,代码终于通过编译器的时候，恍然发现;\n",
	"卧槽，好多功能库都没有;\n",
	"邪教徒来一句，rust是新语言，要积极拥抱新语言,写新项目时抛弃C/C++等旧语言;\n","你说很多库都没有;\n",
	"邪教徒来一句，正是因为这样，是时候为rust做贡献了。;\n",
	"???;\n",
	"..........;\n",
	"..........;\n",
	"就像arch邪教一样的，哈哈哈;\n",
	"另外;\n",
	"我最后想说一句;\n",
	"..........;\n",
	"vb6才是最好的语言;\n"
	];
    for s in &text {
		for c in s.chars() {
			 print!("{}", c);
			 io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(80));
		}
	thread::sleep(Duration::from_millis(500));
	}
    Ok(())
}
