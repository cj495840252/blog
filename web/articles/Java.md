# Java

分析工具 terminate ：`jhsdb hsdb`

ctrl+n搜索类

alt+7罗列大纲

ctrl+F12 罗列大纲，提供搜索

ctrl + alt + v自动生成左边

ctrl+H查看实现类

ctrl + alt + T 用try-catch或其他包围代码块



```java
//相对路径开始的地方，若下列打印出来有module则不用写module，否则需要写
System.out.println(System.getProperty("user.dir"));
```

面向对象的三大特性

封装

继承

**多态**

- 弊端：不能访问子类的特有功能

## 键盘录入

```java
# 第一套体系,遇到空格、回车、制表符停止接收
import java.util.Scanner;
Scanner sc = new Scanner(System.);
int res = sc.nextInt();接受整数
double res = sc.nextDouble();

String line = sc.nextLine();遇到回车停止接收
```

## 生成随机数

```
import java.util.Random;
Random r = new Random;
int number = r.nextInt(101);//生成0-100之间
// 生成m到n之间的随机数
int num = r.nextInt(m-n+1)+m;
```

字符串相加

StringBuilder最快，线程不安全，函数append

StringBuffer差一点，线程安全，函数append

`+`最差，线程安全，一般字符串确定的时候次数少时用

str1.concat(str2) 比+号一点

#### StringBuilder

```
StringBuilder s = new StringBuilder();
// 四个方法
append
reverse反转，无需参数
length
toString
```

## 字符串

```java
String().startsWith("str",index)// 以什么开头
charAt(i);// 取下标为i的值

// 字符串的替换
String s = "aaabbb";
String b = a.replace("aa","ee");// 普通替换不支持正则
// s.replaceAll("要替换的(正则)","替换后的");
// 在底层创建文本解析器对象，循环查找，找到就替换
String res = s.replaceAll("a","b"); ==> // "bbbbbb"
String res = s.replaceAll("(.)\\1+","$1") // 吧重复的替换为单个：ab

字符串的切分
String[] arr = s.split("regex");

// StringBuilder和StringJoin
tm.forEach((k,v)->{System.out.println(new StringBuilder().append(k).append("(").append(v).append(")"));});
StringJoiner sj = new StringJoiner("","","");
tm.forEach((k,v)->sj.add(k+"").add("(").add(v+"").add(")  "));//这里的k为char类型，v为int都需要转String
System.out.println(sj);
```



## 数组

默认初始化类型

```
整数类型：0
小数类型：0.0
字符类型：/u0000 =>k
布尔类型：false
引用数据类型：
```



## 列表

```java
ArrayList<Integer> list = new ArrayList<>();
ArrayList<Integer> list = new ArrayList<>();
Integer x ;
list.add(1);
list.add(10);
Integer res = list.remove(0);//根据下标删除，返回该值
x = 99;
list.set(0,x); // 修改
list.size();//长度
list.get(index);// g获取
```



## 类型转换

判断两个类的类型是否一样

`instanceof`14之间用if

```java
//  jdk14之后
Anamils a = new Dog()
a instanceof Dog b //如果a是Dog类，则转换为Dog，变量名为d
```



### 类型转换

1. **数字转字符串**：123 ==> "123"

​	`Integer.toString(int)`

2. **字符串转数字**："123" ==> 123

​	`Integer.paseInt(s)`

3. **byte数组转String**

​	`String(byte[])`

**打印数组**

相当于print(list),  arr可以为任意数组类型

`Arrays.toString(arr)`

## 静态代码块和构造代码块

```java
public class Student {
    private int id;
    private String name;
    private String sex;
    private int age;
    {
        System.out.println("构造代码块");//每次new student都会执行
    }
    static {
        System.out.println("静态代码块 ");//指挥执行一次，一般数据初始化时使用
    }

    public Student() {
    }
}
```

## 可变参数

相当于python的 **args，

底层为数组

```java
一个函数只能写一个可变参数
可变参数只能写在形参的最后
public function(int...arr) {
}
```



## 接口

```java
public interface Swing {
    int a = 10;  // 会被继承，不能赋值  public final int a =  10；
    public abstract void swim();
	private void fun0()          //不能继承
    public static void fun1(){}  // 可以继承，不能被重写

    // 不重写调用这个，重写后使用重写的
    default void fun2(){System.out.println("this body");}

    // 静态方法不能调用私有方法，所以静态方法需要访问静态私有方法
    private static void fun1(){}  // 可以继承，不能被重写
}
```



## 内部类

```java
public class Outer {
    class Inner{
        int id = 10;
        String name = "陈佳";
    }

    private class Inner1{
        int id = 10;
        String name = "陈佳";
    }
    public Inner1 getInstance(){
        return new  Inner1();
    }

    // 匿名内部类
    new classname/interfacename {
        s
    }
}
public class Main {
    public static void main(String[] args) {
        // 内部类的使用
        Outer.Inner p = new Outer().new Inner();
        System.out.println(p.name);

        Outer p1 = new Outer();
        System.out.println(p1.getInstance());
    }

}
```

## lambda表达式

lambda表达式只能简化函数式接口的匿名内部类的书写

函数式接口：

​	有且仅有一个抽象方法的接口叫函数式接口 ，接口上方可以加@FunctionalInterface注解

```java
public class Main {
    public static void main(String[] args) {
        Integer[] arr = new Integer[]{10,9,8,7,6,5,4,3,2,1};
        // 匿名内部类的书写方式
        Arrays.sort(arr, new Comparator<Integer>() {
            @Override
            public int compare(Integer o1, Integer o2) {
                return o1-o2;
            }
        });
        System.out.println(Arrays.toString(arr));

        // lambda表达式的书写方式
        Arrays.sort(arr,(Integer o1, Integer o2)->{return o2-o1;});// 不止一句时
        Arrays.sort(arr,(Integer o1, Integer o2)-> o2-o1);// 只有一句时
        Arrays.sort(arr,(o1,o2)-> o2-o1);
        System.out.println(Arrays.toString(arr));
    }
}
```

<img src="C:\Users\49584\Desktop\Java\picture\lambda表达式.png" style="zoom:75%;" />

## System

```java
System.exit()// 退出程序
System.currentTimeMillis(); // 当前时间
// 拷贝数组，arr，i1开始，拷贝到arr2，i2开始，拷贝n个,引用数据类型是浅拷贝
System.arraycopy(arr1,i1,arr2,i2,n);
```

## Runtime

```java
Runtime r = Runtime.getRuntime();//不能new，只能get
Runtime.getRuntime();// 静态方法,和上面一样的效果
Runtime.getRuntime().exit();// 静态方法
Runtime.getRuntime().availableProcessors();//线程数
Runtime.getRuntime().maxMemory(); //总内存
Runtime.getRuntime().totalMemory(); // 已经用到
Runtime.getRuntime().freeMemory(); // 还能用的
Runtime.getRuntime().exec("cmd命令"); // cmd窗口
shutdown -s -t 3600 定时关闭单位s
-s默认1m关机
-s -t 指定时间关机
-a 取消关机
-r g
```

## 顶级父类Object

```java
toString()// 默认包名+类名+@+地址值

equals() // 比较类是否相等
String s = "abc";
StringBuilder sb = new StringBuilder("abc");
s.equals(sb);// String中的equals方法，先比较参数是不是字符串，不是直接返回错误
sb.equals(s);//StringBuilder没有提供equals，调用object中的方法，比价地址值
Objects.equals(s, app.s);// 比较对象的属性值

clone() // 克隆类的属性值
类需要实现clone()方法和cloneable接口
该接口为空接口，只是用来做标记
调用时再重新转回该类
public class Test implements Cloneable{
    private int key;
    private String value;
    public Test() {
    }
    @Override
    protected Object clone() throws CloneNotSupportedException {
        return super.clone();
    }
}

Test test = new Test(1,"v1");
Test test1 = (Test) test.clone();//默认Object中的clone也是浅拷贝
Test test2 = test;
test2.setKey(2); // 修改test2中的值，将会修改到test1中的值，浅拷贝
浅克隆：基本数据类型直接拷贝值，引用数据类型拷贝地址值，字符串取串池中找
等号赋值是赋给的地址值，

// 深克隆 gson包
Gson gson = new Gson();
String s = gson.toJson(t1);
System.out.println(gson);
T1 t2 = gson.fromJson(s, t1.getClass());
```



## BigInteger

对象一旦创建，值不能再发生改变

```java
publice BigInteger(int num,Random rnd);// 获取随机大整数，范围[0,2^num-1]
public BigInteger(String val) // 十进制
public BigInteger(Sting val,int radix)//指定进制的大整数
public Static BigInteger valueof(long val)// -16 到 16已经创建好了

// 获取商和余数
BigInteger a = new BigInteger(10,new Random(3));// 获取随机大整数，范围[0,2^num-1]
BigInteger b = new BigInteger(10,new Random(4));// 获取随机大整数，范围[0,2^num-1]
BigInteger[] arr = a.divideAndRemainder(b);
System.out.println(arr[0] +"  "+ arr[1]);

```

<img src="C:\Users\49584\Desktop\Java\picture\BigIntiger.png" style="zoom:80%;" />

## BigDecimal

```java
public BigDecimal(double val) // 这个方法构造是不准确的
public BigDecimal(String val) //建议该方法，否者用这个
    .valueOf // 如果表示的数字不大，没有超过double用这个，0到10之间已经创建好了
```

<img src="C:\Users\49584\Desktop\Java\picture\BigDecimal.PNG" alt="BigD" style="zoom:80%;" />

## 正则表达式

[Java 正则表达式 | 菜鸟教程 (runoob.com)](https://www.runoob.com/java/java-regular-expressions.html)

```java
String str = "";
str.matchs(String regex);

// 额外的 ()包起来的算作一组
"(?i)asDaFasf" // 忽略后面所有的大小写
"as((?i)Da)Fasf" // 忽略da所的大小写
String s = "abcdefg";
s.matches("[a-z]{0,9}"); // true
Java(?=8|11|17);// 匹配java11，Java8，java17 但不匹配java
Java(?!8|11|17);// 匹配除了java11，Java8，java17 之外的java版本

// 贪婪爬取，默认的，尽可能多的匹配
"ab+";
// 非贪婪爬取，尽可能的少获取
"ab+?"; // 问号去掉贪婪

// group 的使用()
// 判断一个字符串开始部分可结束部分是否一致
String s = "aaa123aaa";
// 组2：.任意字符出现一次，组二：组1出现0次或多次，\1最后出现一次组1
s.matches("((.)\\2*).+\1");

// 查找重复连续字符
"(.)\\1+";


String a = "aabbccddaa";
s.matches("((aa)bb).+\2");// \2代表第二组aa
Pattern p = Pattern.compile("((aa)bb)(ccdd)");
Matcher m = p.matcher(a);
while (m.find()){
    System.out.println(m.group(1));aabb
    System.out.println(m.group(2));aa
    System.out.println(m.group(3));ccdd
}
```

Pattern

Matcher

```java
Pattern p = Pattern.compile("正则字符串");
Matcher m = p.matcher(str);
boolean = m.find();// 返回Boolean值
String s1 = m.group();

// 爬取一个网站
URL url = new URL("https://www.cctv.com/entertainment/jingju/yanyuan/xingyun.html");
URLConnection conn = url.openConnection();
BufferedReader br = new BufferedReader(new InputStreamReader(conn.getInputStream(), StandardCharsets.UTF_8));
String line;
while ((line = br.readLine())!=null){
System.out.println(line);
}
br.close();
```



<img src="C:\Users\49584\Desktop\Java\picture\正则表达式.png" alt="正则是" style="zoom:80%;" />

<img src="C:\Users\49584\Desktop\Java\picture\捕获分组.PNG" alt="s" style="zoom:75%;" />

## 时间类

最后：DataTimeFormatter + LocalDateTime + ChronoUnot

JDK7之前

Date  时间

SimpleDateFormate 格式化时间

Calendar 日历，抽象类，不能直接new，getInstance获取

```java
// 按指定格式展示，返回字符串
SimpleDateFormat sdf = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
Date date = new Date(0L);
String d = sdf.format(date);
System.out.println(d);// 1970-01-01 08:00:00
// 指定格式字符串转换成Date
String s = "2022-10-01 19:51:30";
Date d2 =  sdf.parse(s);
System.out.println(d2);// 时间Sat Oct 01 19:51:30 CST 2022
System.out.println(d2.getTime());//时间戳1664625090000



Calendar calendar = Calendar.getInstance();
calendar.getTime();//Mon Oct 03 20:02:33 CST 2022
calendar.getTimeInMillis();//
calendar.get(Calendar.MONTH);// 获取当前月
calendar.getTime();
calendar.getTimeInMillis();
calendar.get(Calendar.MONTH);// 月份0-11表示，需要加1
calendar.get(Calendar.DAY_OF_MONTH);// 月第几天
calendar.get(Calendar.DAY_OF_WEEK);// 周几，星期天为第一天，加一
calendar.toInstant();//减去8h
```

<img src="C:\Users\49584\Desktop\Java\picture\日历.PNG" style="zoom:75%;" />

### JDK8时间类

<img src="C:\Users\49584\Desktop\Java\picture\JDK8时间类.png" style="zoom:55%;" />

ZoneID时区

```java
Set<String> zoneIds = ZoneId.getAvailableZoneIds();// 所有时区的集合
ZoneId z = ZoneId.systemDefault();//系统默认时区
ZoneId.of("Asia/chongqiong");//获取指定时区
System.out.println(z);
```



<img src="C:\Users\49584\Desktop\Java\picture\ZoneId时区.png" style="zoom:50%;" />

Instant时间戳

```java
Instant now = Instant.now();// 获取当前标准时间 2022-10-03T12:35:48.391126100Z
Instant instant1 = Instant.ofEpochMilli(0L);// 时间戳获取时间 1970-01-01T00:00:00Z
Instant instant2 = Instant.ofEpochSecond(60L); //时间戳过了参数秒的时间 1970-01-01T00:01:00Z
ZonedDateTime time = Instant.now().atZone(ZoneId.of("Asia/Shanghai"));// 2022-10-03T20:38:55.947335900+08:00[Asia/Shanghai]
boolean res = instant1.isAfter(instant2);//false, isAfter在...什么之前,isBefore 在...之后
```



![](C:\Users\49584\Desktop\Java\picture\Instant时间戳.png)

ZoneDateTime

```java
ZonedDateTime now = ZonedDateTime.now();// 获取当前时间带时区
ZonedDateTime time1 = ZonedDateTime.of(2022,10,1,11,12,12,0,ZoneId.of("Asia/Shanghai"));
Instant instant = Instant.ofEpochMilli(0L);
ZoneId zoneId = ZoneId.of("Asia/Shanghai");

ZonedDateTime time2 = ZonedDateTime.ofInstant(instant,zoneId);// 生成时间
ZonedDateTime time3 = time2.with(ChronoField.YEAR,2021);//修改时间
ZonedDateTime time4 = time3.minusDays(100);// 减少时间
ZonedDateTime time5 = time3.plusYears(1);// 增加时间

System.out.println(now);//2022-10-03T20:51:22.299528700+08:00[Asia/Shanghai]
System.out.println(time1);//2022-10-01T11:12:12+08:00[Asia/Shanghai]
System.out.println(time2);//1970-01-01T08:00+08:00[Asia/Shanghai]
System.out.println(time3);//2021-01-01T08:00+08:00[Asia/Shanghai]
System.out.println(time4);//2020-09-23T08:00+08:00[Asia/Shanghai]
System.out.println(time5);//2022-01-01T08:00+08:00[Asia/Shanghai]
```

时间格式化DateTimeFormatter

```java
ZonedDateTime now = Instant.now().atZone(ZoneId.of("Asia/Shanghai"));// 获取当前时间带时区
DateTimeFormatter dtf1 = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");
System.out.println(dtf1.format(now));// 2022-10-03 21:00:45
```



LoacalDate     localTime    localDateTime

```java
LocalDate nowDate = LocalDate.now();//当前时间的日历对象
LocalDate ldDate = LocalDate.of(2023,1,1);//指定时间的日历对象
int year = ldDate.getYear();//get方法获取日历中的信息
int month = ldDate.getMonthValue();
int day = ldDate.getDayOfMonth();
DayOfWeek dayOfWeek = ldDate.getDayOfWeek();// SUNDAY英文星期
nowDate.isAfter(ldDate);
nowDate.isBefore(ldDate);
```



<img src="C:\Users\49584\Desktop\Java\picture\Jdk8的日历类.png" style="zoom:75%;" />

Duration、Period、ChronoUnit

```java
LocalDate date = LocalDate.of(2022,10,1);
LocalDate now = LocalDate.now();
Period period = Period.between(date,now);//第二个参数减去第一个参数：P2D
System.out.println(period);

LocalDateTime now1 = LocalDateTime.now();
LocalDateTime local = LocalDateTime.of(2021,10,3,0,0,0);
Duration diff = Duration.between(now1, local);//PT-21H-35M-2.7473911S
System.out.println(diff);//to方法转换成秒分钟小时等

// 重点
System.out.println("相差的年数"+ ChronoUnit.YEARS.between(local,now1));
System.out.println("相差的月数"+ ChronoUnit.MONTHS.between(local,now1));
System.out.println("相差的周数"+ ChronoUnit.WEEKS.between(local,now1));
System.out.println("相差的天数"+ ChronoUnit.DAYS.between(local,now1));
System.out.println("相差的时数"+ ChronoUnit.HOURS.between(local,now1));
System.out.println("相差的分数"+ ChronoUnit.MINUTES.between(local,now1));
System.out.println("相差的秒数"+ ChronoUnit.SECONDS.between(local,now1));
System.out.println("相差的毫秒数"+ ChronoUnit.MILLIS.between(local,now1));
System.out.println("相差的微秒数"+ ChronoUnit.MICROS.between(local,now1));
System.out.println("相差的纳秒数"+ ChronoUnit.NANOS.between(local,now1));
System.out.println("相差的半天数"+ ChronoUnit.HALF_DAYS.between(local,now1));
System.out.println("相差的十年数"+ ChronoUnit.DECADES.between(local,now1));
System.out.println("相差的世纪数"+ ChronoUnit.CENTURIES.between(local,now1));
System.out.println("相差的千年数"+ ChronoUnit.MILLENNIA.between(local,now1));
System.out.println("相差的纪元数"+ ChronoUnit.ERAS.between(local,now1));
```





<img src="C:\Users\49584\Desktop\Java\picture\Duration-Period-ChronoUnit.png" style="zoom:50%;" />

## 查找方法

七种查找方式

基本查找，二分查找，插值查找，斐波那契查找，分块查找，查哈希查找，树表查找

```java
public static void main(String[] args) throws IOException, ParseException {
	int[] arr = {11,2,30,91,88,5,21,54,30,9,78,22,15,1};
	int a = 30;

	// 顺序查找
	System.out.println(orderfind(arr,a));

	// 二分查找,有序
	System.out.println(halffind(new int[]{1,2,3,4,5,6,7},7));
}

// 顺序查找
public static List<Integer> orderfind(int[] arr, int value){
	ArrayList<Integer> list = new ArrayList<>();
	for (int i = 0; i < arr.length; i++) {
		if (arr[i]==value){
			list.add(i);
		}
	}
	return list;
}

// 二分查找
public static int halffind(int[] arr,int value){
	int min = 0;
	int max = arr.length-1;
	while (min <= max){
		int mid = (max+min)/2;
		if (arr[mid]>value){
			max = mid - 1;
		}else if (arr[mid] < value){
			min = mid + 1;
		}else {
			return mid;
		}
	}
	return -1;
}


// 分块查找，分块的个数一般为数组长度开方
// todo 代码太麻烦，不实现
static class Block{
	int max;//块中最大值
	int startIndex;// 起始索引
	int endIndex; // 结束索引
}
```



## 排序方法

冒泡排序

```java
public static int[] maopao(int[] arr){
    for (int i = 0; i < arr.length-1; i++) { // 由于最后一轮只有一个值不用排
        for (int j = 0; j < arr.length-i-1; j++) {
            if (arr[j] > arr[j+1]){
                int temp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = temp;
            }
        }
    }
    return arr;
}
```



选择排序

```java
public static int[] selectsort(int[] arr){
    int temp ;
    boolean f = false;
    for (int i=0;i< arr.length-1;i++){
        f = true;
        for (int j = i + 1; j < arr.length; j++) {
            if (arr[i] > arr[j]){
                temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
                f = false;
            }
        }
        if (f)break;
    }
    return arr;
}
```

插入排序

```java
public static int[] insersort(int[] arr){
    int start = -1;
    for (int i = 0; i < arr.length-1; i++) {
        if (arr[i] > arr[i+1]) {
            start = i + 1;
            break;
        }
    }
    for (int i = start; i < arr.length; i++) {
        int j = i;
        while (j > 0 && arr[j] < arr[j-1]){
            int temp = arr[j];
            arr[j] = arr[j-1];
            arr[j-1] = temp;
            j --;
        }
    }
    return arr;
}
```

快速排序

```JAVA
// 快速排序
public static int[] quicksort(int[] arr,int left,int right){
    if (left<right){
        int i = onequick(arr,left,right);
        quicksort(arr,left,i-1);
        quicksort(arr,i+1,right);
    }
    return arr;
}
public static int onequick(int[] arr,int start,int end){
    int poiet = arr[start];
    while (end > start){
        while (end > start && arr[end]>=poiet) end--;
        if (arr[end] < poiet){
            arr[start] = arr[end];
            start++;
        }
        while (end > start && arr[start]<=poiet)start++;
        if (arr[start] > poiet){
            arr[end] = arr[start];
            end --;
        }
    }
    arr[start] = poiet;
    return start;
}

public static void quickSort(int[] arr, int left, int right){
    if (left < right){
        int i = left,j=right,pivot=arr[left];
        while (i<j){
            while (i<j && arr[j] >=pivot)j--;
            if (arr[j]<pivot){
                arr[i] = arr[j];
                i++;
            }
            while (i<j && arr[i] <= pivot)i++;
            if (arr[i]>pivot){
                arr[j]=arr[i];
                j--;
            }
        }
        arr[i] = pivot;
        quickSort(arr,left,i-1);
        quickSort(arr,i+1,right);
    }
}
```



## Arrays

操作数组的工具类

```java
T1[] t1s = new T1[10];
for (int i = 0; i < t1s.length; i++) {
    t1s[i] = new T1(10-i);
}
for (int i = 0; i < t1s.length; i++) {
    System.out.println(t1s[i]);
}
// sort方法是二分查找，在有序数列中插入数据，
// 循环遍历获取到要插入的元素a
// 二分查找，找到插入点
// a和插入点元素比较
// a小继续跟前面一个比较，直到a大插入两个之间
// a大继续跟后面一个比较，直到a小，插入两个之间
// 自定义比较函数返回int类型确定大小
Arrays.sort(t1s,Main::compare);//compare
System.out.println(Arrays.toString(t1s));


public static int compare(T1 a,T1 b){ // 第一个大于第二个参数就是升序
    return a.getValue()-b.getValue();
}
```



<img src="C:\Users\49584\Desktop\Java\picture\Arrays.png" alt="a" style="zoom:75%;" />





## 单列顶层Collection类

单列：

双列：键值对

collection类型可以直接打印

```java
Collection<String> coll = new ArrayList<>();
//添加元素，返回boolean类型
// 只有set类型的集合添加重复元素时才返回false
coll.add("aaa");
coll.add("bbb");
coll.add("ccc");
coll.isEmpty();
//coll.clear();
//删除不存在的时候返回false
coll.remove("aaa");
coll.contains("bbb");//true,如果list中是class，需要重写equals方法


// 列表嵌套
Collection<Collection<String>> lists = new ArrayList<>();
lists.add(coll);
System.out.println(lists);
```

### 遍历方式

迭代器遍历：Iterator

- 迭代器遍历时不能用集合方式增加和删除，
- 迭代器 方式删除，it.remove ()， 不支持添加

增强for遍历

- for遍历时，修改变量的值不会改变集合中原来的数据

```java
// 迭代器遍历
Collection<String> coll = new ArrayList<>();
coll.add("aaa");
coll.add("bbb");
coll.add("ccc");
coll.add("DDD");
coll.add("ggg");
coll.add("lll");

Iterator<String> it = coll.iterator();
while (it.hasNext()){
    it.remove()
	System.out.println(it.next());
}

// 增强for遍历
for (String s : coll) {
    System.out.println(s);
}
// lmbda表达式遍历
coll.forEach(e->{
    System.out.println(e);
});
```



<img src="C:\Users\49584\Desktop\Java\picture\collecction.png" alt="a" style="zoom:75%;" />

<img src="C:\Users\49584\Desktop\Java\picture\collection的方法.png" alt="functio" style="zoom:75%;" />

### List

```java
List<String> list = new ArrayList<>();
list.add("aaa");
list.add("bbb");
list.add("ccc");
list.add("DDD");
list.add("ggg");
list.add("lll");
list.add(1,"zzz");//在index之后插入一个元素
list.remove(1);//删除下标索引为index的值，从0开始
list.set(2,"CCC"); // 修改索引为index的值
String res = list.get(5);// 获取指定索引的值
System.out.println(res);// lll
list.forEach((e)-> System.out.println(e));


List<Integer> list = new ArrayList<>();
list.add(1);
list.add(2);
list.add(3);
// 删除元素1，删除时默认调用实参跟形参相同的方法
list.remove(0);
list.remove(Integer.valueOf(1));
list.forEach((e)-> System.out.println(e));
```

**list的遍历方式**

1.迭代器

2.列表迭代器

- 增加了指针向前移动的方法
- 增加了添加元素的方法，在当前指针后面添加

3.增强for

4.Lambda表达式

5.普通for

```java
List<String> list = new ArrayList<>();
list.add("aaa");
list.add("bbb");
list.add("ccc");
list.add("DDD");
list.add("ggg");
list.add("lll");

// 迭代器
Iterator<String> it = list.iterator();
while (it.hasNext()){
	System.out.println(it.next());
}

// 列表迭代器
ListIterator<String> it = list.listIterator();
while (it.hasNext()){
    if (it.next() == "ccc"){
        it.add("CCC");
    }
}
list.forEach(e-> System.out.println(e));
// 增强for
for (String s : list) {
	System.out.println(s);
}
// lambda表达式
list.forEach(System.out::println);
list.forEach(new Consumer<String>() { // 完整形式
    @Override
    public void accept(String s) {
        System.out.println(s);
    }
});

// 普通for
for (int i = 0; i < list.size(); i++) {
	System.out.println(list.get(i));
}
```



<img src="C:\Users\49584\Desktop\Java\picture\list特有方法.png" style="zoom:75%;" />

<img src="C:\Users\49584\Desktop\Java\picture\list遍历对比.png" style="zoom:75%;" />

**ArrayList**

- 空参构造时创建一个默认长度为0的数组

- 添加一个元素时，底层创建一个新的长度为10的的数组

- 存满时，扩容1.5倍

- 如果一次添加多个，扩容实际长度

**LinkkedList**

- 底层数据结构时双链表
- 提供了操作首尾元素特有的API

<img src="C:\Users\49584\Desktop\Java\picture\LinkedList.png" style="zoom:75%;" />

<img src="C:\Users\49584\Desktop\Java\picture\LinkedList代码分析.png" style="zoom:75%;" />



### Set

- set系列集合的特点

  - 没有额外的方法
  - 无序、不重复、无索引

- 实现类特点

  数组+链表+红黑树

  $\color{#ff0000}{链表长度超过8,而且数组长度大于等于64时，自动转换为红黑树}$

  根据hashCode和equals方法去重

  - HashSet：无序、不重复、无索引

  - LinkedHashSet：有序、不重复、无索引

    增加了双链表机制，记录存取顺序，保证存取顺序一致

  - TreeSet：可排序、不重复、无索引 (自动排序)

```java
        Set<String> set = new HashSet<>();
        set.add("陈佳");
        set.add("陈佳");
        set.add("张三");

        // 迭代器遍历
        Iterator<String> it = set.iterator();
        while (it.hasNext()){
            System.out.println(it.next());
        }
        // 增强for
        for (String s : set) {
            System.out.println(s);
        }
        // lambda表达式
        set.forEach((e)-> System.out.println(e));
```

<img src="C:\Users\49584\Desktop\Java\picture\set.png" style="zoom:75%;" />

**LinkedHashSet**

有序、不重复、无索引

增加了双链表机制，记录存取顺序，保证存取顺序一致

<img src="C:\Users\49584\Desktop\Java\picture\LinkedHashSet.png" style="zoom:67%;" />

**TreeSet**

自定义类排序规则

默认使用第一种，如果第一种不能满足需求时，就使用第二种

方式一

- 默认排序/自然排序：实现Comparable接口，重写抽象方法

  this：代表要添加的元素

  o：代表已经再红黑树中的元素

方式二

- 比较器排序

```java
// =====================================方式一=========================================================
public class Student implements Comparable<Student>{
    private int age;
    private String name;

    public Student() {
    }

    public Student(int age, String name) {
        this.age = age;
        this.name = name;
    }

    // 重写接口方法
    @Override
    public int compareTo(Student o) {
        return this.getAge() - o.getAge();
    }
}

// Main方法中
TreeSet<Student> set = new TreeSet<>();
Student s1 = new Student(1,"陈佳");
Student s2 = new Student(2,"张三");
Student s3 = new Student(3,"李四");

set.add(s2);
set.add(s3);
set.add(s1);

// =====================================方式二=========================================================
public static void main(String[] args) {
    // 需求，按字符串长度排序，再按ASCII排序
    // o1表示当前要添加的元素
    // o2表示红黑树中已经存在的元素
    TreeSet<String> set = new TreeSet<>(new Comparator<String>() {
        @Override
        public int compare(String o1, String o2) {
            int i = o1.length()-o2.length();
            i = i==0?o1.compareTo(o2) : i;
            return i;
        }
    });

//    TreeSet<String> set = new TreeSet<>(( o1, o2)-> {
//        int i = o1.length()-o2.length();
//        i = i==0?o1.compareTo(o2) : i;
//        return i;
//    });

    set.add("ac");
    set.add("ab");
    set.add("bcd");
    set.add("a");

    System.out.println(set);// [a, ab, ac, bcd]

}
```

### collections

单列集合的工具类

```java
public static void main(String[] args) {
    ArrayList<String> list = new ArrayList<>();
    // 1.批量添加
    Collections.addAll(list,"a","b","c");
    // 2.打乱
    Collections.shuffle(list);
    // 3.sort
    Collections.sort(list);
    // 4.copy
    ArrayList<Object> list1 = new ArrayList<>();
    Collections.copy(list1,list);
    // 5.fill
    Collections.fill(list,"a");
    // 6.max
    Collections.max(list, (a,b)->{
        int i = a.charAt(0) - b.charAt(0);
        return i;
    });
    // 7.swap
    Collections.swap(list,0,1);
}
```



## 双列顶层Map

- LinkedHashMap继承HashMap，保证存取有序
- TreeMap提供对key排序，自定义时同样需要实现comparable接口
- 不要求排序时，选择HashMap效率最高

### HashMap

1. 默认初始化为null
2. 如果是第一次put时，将会创建一个长度为16的数组，加载因子0.75
3. 如果是不是第一次put时，先判断是否需要扩容，不需要则什么都不做
4. 需要扩容时，将会把数组扩容为两倍，并且添加到新的HashMap中
5.

**基本操作**

```java
Map<String,String> m = new HashMap<>();
// 在添加数据时，如果键不存在，直接添加，返回null
// 存在时，则覆盖，返回之前的value
m.put("郭靖","黄蓉");
m.put("韦小宝","沐剑屏");
m.put("尹志平","小龙女");
m.containsKey("郭靖"); // 返回Boolean
m.containsValue("小龙女"); // 返回Boolean
String rem = m.remove("尹志平"); // 小龙女
String value = m.put("韦小宝","双儿"); //value = 沐剑屏
```



<img src="C:\Users\49584\Desktop\Java\picture\Map的Api.png" style="zoom:75%;" />

**遍历方式**

```JAVA
Map<String, String> m = new HashMap<>();
// 在添加数据时，如果键不存在，直接添加，返回null
// 存在时，则覆盖，返回之前的value
m.put("郭靖", "黄蓉");
m.put("韦小宝", "沐剑屏");
m.put("尹志平", "小龙女");

// 通过键找值
// 获取所有的键，返回单列集合
Set<String> set = m.keySet();
for (String key : set) {
    System.out.println(m.get(key));
}
Iterator<String> it = set.iterator();
while (it.hasNext()){
    System.out.println(m.get(it.next()));
}
set.forEach(e-> System.out.println(m.get(e)));

// 通过键值对遍历
Set<Map.Entry<String, String>> entries = m.entrySet();//Entry：外部接口中的内部接口
for (Map.Entry<String, String> entry : entries) {
    System.out.println(entry.getKey()+" -> "+entry.getValue());
}
Iterator<Map.Entry<String, String>> it2 = m.entrySet().iterator();
while (it2.hasNext()){
    Map.Entry<String, String> entry = it2.next();
    System.out.println(entry.getKey()+ " -> " + entry.getValue());
}
entries.forEach(e-> System.out.println(e.getKey()+"-->"+e.getValue()));

// 直接遍历,实际上调用的第二种方法
m.forEach((k,v) -> {
    System.out.println(k +" -> "+ v);
});
```

### TreeMap降序

```java
// 按照key降序
Map<Integer, String> m = new TreeMap<>(new Comparator<Integer>() {
    @Override
    public int compare(Integer o1, Integer o2) {
        return o2-o1;
    }
});
// 在添加数据时，如果键不存在，直接添加，返回null
// 存在时，则覆盖，返回之前的value
m.put(3, "黄蓉");
m.put(6, "沐剑屏");
m.put(4, "小龙女");

System.out.println(m);


// TreeMap排序实现第一种自定义对象的自然排序
@Override
public int compareTo(Student o) {
    // 按照学生年龄升序排序，年龄一样按照姓名的字母排序，同姓名年龄视为同一个人
    // this: 当前要添加的元素
    // o: 表示已经在红黑树中的元素

    // 返回值：
    // 负数：表示当前要添加的元素是小的，存左边
    // 正数：表示当前要添加的元素是大的，存右边
    // 0: 表示当前元素已存在，舍弃
    int i = this.getAge() - o.getAge();
    i= i==0?this.getName().compareTo(o.getName()):i;
    return i;
}


// 字符计数，StringBuilder和StringJoiner
String s = "aahsflkasdhfuhasdashgdjashdashfksalfhsdkjfszlkdjhfzsdjfh";
TreeMap<Character,Integer> tm = new TreeMap<>();
for (int i = 0; i < s.length(); i++) {
    char c = s.charAt(i);
    if (tm.containsKey(c)){
        int count = tm.get(c);
        count++;
        tm.put(c,count);
    }else {
        tm.put(c,1);
    }
}

System.out.println(tm);
tm.forEach((k,v)->{System.out.println(new StringBuilder().append(k).append("(").append(v).append(")"));});
StringJoiner sj = new StringJoiner("","","");
tm.forEach((k,v)->sj.add(k+"").add("(").add(v+"").add(")  "));//这里的k为char类型，v为int都需要转String
System.out.println(sj);
```

### 不可变Map

```java
HashMap<String, String> hashMap = new HashMap<>();
hashMap.put("k1","v1");
hashMap.put("k2","v2");
hashMap.put("k3","v3");
hashMap.put("k4","v4");
hashMap.put("k5","v5");
hashMap.put("k6","v6");
hashMap.put("k7","v7");
hashMap.put("k8","v8");
hashMap.put("k9","v9");
hashMap.put("k10","v10");
hashMap.put("k11","v11");

// 获取键值对
Set<Map.Entry<String, String>> entries = hashMap.entrySet();
// 将键值对转换为数组
Map.Entry[] arr = new Map.Entry[0];
//toArray方法会比较数组的长度，返回和arr一样长度的数组
entries.toArray(arr);
Map map = Map.ofEntries(arr);


// 简化方法
hashMap.entrySet().toArray(new Map.Entry[0]);
```

## 泛型

> java中的泛型为伪泛型，只在编译阶段检查

格式 <数据类型>

- 泛型只能支持$\color{#ff0000}{引用数据类型}$
- 创建集合时，不指定泛型将默认为Object类型，在获取数据时不能使用她的的特有属性

```java
//定义一个泛型类，这个类似list
public class T1<E>{
    Object[] obj = new Object[10];
    int size;
    public boolean add(E e){
        obj[size] = e;
        size++;
        return true;
    }

    public E get(int index){
        return (E)obj[index];
    }

    @Override
    public String toString() {
        return Arrays.toString(obj);
    }
}

// 	可变参数
public static <E>  void addAll(ArrayList<E> list, E...e){
    for (int i = 0; i < e.length; i++) {
        list.add(e[i]);
    }
}
ArrayList<Integer> list0 = new ArrayList<>();
list0.addAll(Arrays.asList(new Integer[]{1,2,3,4,5,6,7}));
T1.addAll(list0,1,2,3,4,5,8);// 可变参数的调用方式

public static <E>  void addAll(ArrayList<E> list, ArrayList<E> list2){
        list.addAll(list2);
}
```

**泛型接口**

```java
修饰符 interface 接口名<类型>{}

当实现类继承泛型接口时，指定类型后，new时不需要再指定泛型
public class  T1 implements List<String>{}

当实现类不实现泛型时
public class myArrayList<E> implements List<E>{}

// 泛型不具备继承性，但是数据具备继承性
定义三个类，ye，fu，zi；fu继承ye，zi继承fu
public static void method(ArrayList<ye> list){}//只能传递ye,不能传递子类
// 添加元素时添加子类数据
ArrayList<ye> list1 = new ArrayList<>();
ArrayList<fu> list2 = new ArrayList<>();
ArrayList<zi> list3 = new ArrayList<>();
list1.add(new fu());
list1.add(new zi());

// 限制泛型
public static void method(ArrayList<?extends ye> list){}//限制只能存放该类及其子类
public static void method(ArrayList<?super fu> list){}//限制只能存放该类及其父类
```





## 平衡二叉树

二叉查找树：小的存左边，大的存右边，一样的不存

### 左旋

<img src="C:\Users\49584\Desktop\Java\picture\左旋2.png" style="zoom:45%;" />

### 右旋

 <img src="C:\Users\49584\Desktop\Java\picture\右旋.png" style="zoom:45%;" />

### 导致不平衡的四种情况

**左左**

<img src="C:\Users\49584\Desktop\Java\picture\左左.png" style="zoom:55%;" />

**左右**

<img src="C:\Users\49584\Desktop\Java\picture\左右.png" style="zoom:55%;" />

**右右**

<img src="C:\Users\49584\Desktop\Java\picture\右右.png" style="zoom:55%;" />

<img src="C:\Users\49584\Desktop\Java\picture\右右2.png" style="zoom:55%;" />

**右左**

<img src="C:\Users\49584\Desktop\Java\picture\左右.png" style="zoom:55%;" />

### 总结

<img src="C:\Users\49584\Desktop\Java\picture\四种旋转.png" style="zoom:45%;" />

## 红黑树

- 是一个二叉查找树
- 不是高度平衡
- 特有的红黑规则
- 添加节点时默认添加红色

**红黑规则**

- 每一个节点是红色或黑色
- 根节点必须黑色
- 如果一个节点没有子节点或则父节点，则该节点相应的指针苏醒值为Nil, 这些Nil视为叶子节点，每个叶节点是黑色的
- 红节点的子节点必须为黑色的（不能出现两个红的连在一起）
- 对每一个节点，从该节点到其所有后代叶节点的简单路径上，均包含相同数目的黑色节点

<img src="C:\Users\49584\Desktop\Java\picture\红黑树.png" style="zoom:75%;" />

<img src="C:\Users\49584\Desktop\Java\picture\红黑树规则.png" style="zoom:55%;" />

### 红黑树添加节点规则

- 默认添加红色节点

<img src="C:\Users\49584\Desktop\Java\picture\红黑树添加节点规则.png" style="zoom:75%;" />

## 方法引用

在类中非静态方法中引用本类非静态方法：this::方法名

在类中非静态方法中引用父类非静态方法：supper::方法名

在本类中静态方法引用非静态方法：创建本类或父类

引用构造方法：类名::new





## 异常

![](C:\Users\49584\Desktop\Java\picture\异常体系.png)

> 异常的三个方法
>
> `public String getMessage()`:返回throwable的详细消息字符串
>
> `public String toString()`:返回此可抛出的简短描述
>
> `public void printStackTrace`:把异常的错误信息输出到控制台



### 异常抛出

![](C:\Users\49584\Desktop\Java\picture\异常抛出.png)





# IO流

## File

> `File`：表示文件路径，可以是文件，也可以是文件夹，可以操作该文件夹下的文件
>
> 这个路径可以存在，也可以不存在
>
> ```java
> //三种构造方法
> File file1 = new File("C:\\");
> File file2 = new File("/root","./home");
> File file3 = new File(file1,"./project");
> ```

### 成员方法

#### 判断和获取

| 方法名                            | 说明                     |
| --------------------------------- | ------------------------ |
| `public boolean isDirectory()`    | 判断该路径是否为文件夹   |
| `public boolean isFile()`         | 判断该路径是否为文件     |
| `public boolean exists`           | 判断该路径是否存在       |
| `public long length()`            | 返回文件的大小（字节数） |
| `public String getAbsolutePath()` | 返回绝对路径             |
| `public String getPath()`         | 返回定义文件时使用路径   |
| `public String getName()`         | 返回文件名，带后缀       |
| `public long lastModified()`      | 返回上一次修改时间       |

#### 创建和删除

`File文件路径不存在，且父级路径存在才能创建`

| 方法名称                         | 说明                     |
| -------------------------------- | ------------------------ |
| `public boolean createNewFile()` | 创建一个新的文件         |
| `public boolean mkdir()`         | 创建单级文件夹           |
| `public boolean mkdirs()`        | 创建多级文件夹           |
| `public boolean delete()`        | 删除文件或者删除空文件夹 |

#### 遍历

| 方法名称                                         | 说明                                            |
| ------------------------------------------------ | ----------------------------------------------- |
| `public File[] listFiles()`                      | 获取当前路径下所有内容,无权限和空文件夹返回null |
| `public static File[] Roots()`                   | 列出可用的系统根，C，D，E                       |
| `public String[] list()`                         | 获取当前文件夹下所有内容,                       |
| `public String[] list(FilenameFilter filter)`    | 过滤器过滤文件                                  |
| `public File[] listFiles(FileFilter filter)`     | 过滤器过滤文件                                  |
| `public File[] listFiles(FilenameFilter filter)` | 过滤器过滤文件                                  |

```java
//查找文件夹f下所有txt文件
// 方法一
File f = new File("D:\\");
File[] arr = f.listFiles();
for (File file: arr){
    if(file.isFile() && file.getName().endWith(".txt")){
        System.out.println(file)
    }
}

// 方法二，dir为构造时的输入路径，name为该路径下的文件或文件夹
String[] list = f.list(new FilenameFilter() {
            @Override
            public boolean accept(File dir, String name) {
                File src = new File(dir,name);
                return src.isFile() && name.endsWith(".txt");
            }
        });
// 方法三,参数为绝对路径，等于dir+n
File[] files = file1.listFiles(new FileFilter() {
            @Override
            public boolean accept(File pathname) {
                return pathname.isFile() && pathname.getName().endsWith(".txt");
            }
        });
```





## Stream流

配合集合和数组的使用

```java
// 单列集合获取stream
ArrayList<String> list = new ArrayList<>();
Collections.addAll(list,"a","b","c");
list.stream().forEach(System.out::println);

// 双列集合获取stram
// 第一种
HashMap<String, String> hashMap = new HashMap<>();
hashMap.keySet().stream().forEach(System.out::println);
//第二种
hashMap.entrySet().stream().forEach(System.out::println);


//数组
int[] arr = {1,2,3,4,5};
Arrays.stream(arr).forEach(System.out::println);

// 直接获取一个流,不要传递基本数据类型数组，不然当成一个元素
Stream<Integer> stream = Stream.of(1, 2, 3, 4, 5);
stream.forEach(System.out::println);
```

### 中间方法

<img src="C:\Users\49584\Desktop\Java\picture\Stream流中间方法.png" style="zoom:67%;" />

- map方法：T转换前的类型，R转换后的类型

### 终结方法

```java
// 直接获取一个流,不要传递基本数据类型数组，不然当成一个元素
Stream<Integer> stream = Stream.of(1, 2, 3, 4, 5);

// foreach

// count

// toAarry()方法的参数：负责创建一个指定类型的数组
// 会依次的到流里面的数据，放到数组中
stream.toArray(new IntFunction<String[]>() {
    @Override
    public String[] apply(int value) {
        return new String[value];
    }
});


//reduce,自定义聚合方法
Integer integer = stream.reduce(new BinaryOperator<Integer>() {
    @Override
    public Integer apply(Integer integer, Integer integer2) {
        return integer + integer2;
    }
}).get();
System.out.println(integer);



// 收集方法
stream.collect(Collectors.toList());
stream.collect(Collectors.toSet());

// Function中泛型1：表示流中每个数据的类型
// Function中泛型1：表示收集到Map集合中时数据的类型
// apply方法中参数表示流中的数据，收集时键不能重复
stream.collect(Collectors.toMap(new Function<Integer, String>() {
    @Override
    public String apply(Integer integer) {
        return String.valueOf(integer);
    }
}, new Function<Integer, Object>() {
    @Override
    public Object apply(Integer integer) {
        char c = 'a';
        return String.valueOf(c+integer);
    }
}));
```



### IO流

用记事本打开能读懂的就是纯文本文件

![](C:\Users\49584\Desktop\Java\picture\io流的分类.png)

####  体系

![](C:\Users\49584\Desktop\Java\picture\io流体系.png)

#### InputStream

**该类由两个子类**

都具有一样的方法

**FileInputStream**

```java
public class Out {
    public static void main(String[] args) throws IOException {
        //创建输入流对象
        FileInputStream fis = new FileInputStream("");
        //一个字节一个字节的方式读取
        int read1 = fis.read();
        System.out.println((char)read1);
        byte[] arr = new byte[10];
        int a = fis.read(arr,0,5);//一次读取多个
        System.out.println(a); //返回数组下标
        System.out.println(Arrays.toString(arr));
        //关闭流
        fis.close();
    }
}
```

#### OutputStream

**FileOutputStream**

```java
public class IOStream {
    public static void main(String[] args) throws IOException {
        // 这里的true表示是否追加
        FileOutputStream f = new FileOutputStream("",true);
        f.write("陈佳".getBytes(StandardCharsets.UTF_8));
        String s = "123adjkas";//只能一个一个字符的形式写入
        f.write(s.getBytes());
        f.close();
    }
}
```

#### BufferedOutputStream

```java
public class Buffered {
    public static void main(String[] args) throws Exception {
        BufferedOutputStream buf = new BufferedOutputStream(Files.newOutputStream(Paths.get("./module1/iostream/aa.txt")));
        buf.write("abcd".getBytes());
        buf.close();
    }
}
```

#### BufferedInputStream

```java
public class BufferIS {
    public static void main(String[] args) throws IOException {
        BufferedInputStream buf = new BufferedInputStream(new FileInputStream("path"));
        byte[] arr = new byte[10];
        buf.read(arr);
        System.out.println(new String(arr));
        buf.close();
    }
}
```



### 序列化

`将一个student类序列化，然后给类添加一个字段，在将序列化到文件中的类读取出来`

#### ObjectOutputStream

```java
@AllArgsConstructor
@NoArgsConstructor
@Data
public static class Student implements Serializable{
    public static final long serialVersionUID = 1L;
    String name;
    int age;
    String sex;
    //String info;
    private transient int t;// 使用transient修饰的变量序列化时会被去掉
}


Student student = new Student("陈佳", 23, "男");
Student student1 = new Student("李四",24,"女");
FileOutputStream fileOutputStream = new FileOutputStream("./data/a.txt");
ObjectOutputStream out = new ObjectOutputStream(fileOutputStream);
out.writeObject(student);
out.writeObject(student1);
out.close();
```

#### ObjectInputStream

```java
FileInputStream fileInputStream = new FileInputStream("./data/a.txt");
ObjectInputStream in = new ObjectInputStream(fileInputStream);
Object o ;
try {
    while ( (o = in.readObject())!= null) {
        Student s = (Student) o;
        System.out.println(s);
    }
}catch (EOFException e){
    System.out.println("文件读取完毕");
    in.close();
}

```

### 打印流

`用来写出用的，原样写出`

#### 字节打印流

`无缓冲区,第二个参数代表自动刷新，所以无意义`

```java
PrintStream stream = new PrintStream("./data/a.txt");
//FileOutputStream out = new FileOutputStream(new File("./data/a.txt"))
//PrintStream stream1 = new PrintStream(out, true, StandardCharsets.UTF_8);
stream.println("陈佳");
stream.close();
```

#### 字符打印流

`第二个参数代表自动刷新`

```java
FileOutputStream fos = new FileOutputStream(new File("data/a.txt"));
PrintWriter w = new PrintWriter(fos,true, Charset.forName("utf-8"));
w.println("爱意东升西落");
w.close();
fos.close();

```



# 常用的jar

### commons-io：输入输出工具类

```java
public class FileU {
    public static void main(String[] args) throws IOException {
        ArrayList<String> list = new ArrayList<>();
        list.add("abc");
        list.add("陈佳");
        list.add("王五");

        FileUtils.writeLines(new File("path"),list,true); // 写入一个数组
        FileUtils.write(new File("path"),"abcdrasflhsfkjaf");  // 写入String类型
        String s = FileUtils.readFileToString(new File("path"));  // 读出
        byte[] arr = FileUtils.readFileToByteArray(new File("path"));
        System.out.println(s);
        System.out.println(new String(arr));
    }
}
```



## junit：单元测试用

@Test修饰：没有main时可以运行该方法，

@Before修饰：@Test修饰函数之前自动运行

@After修饰：@Test修饰之和自动运行

**被修饰的函数，不能有返回值，不能传递参数**

```java
public class Test_junit {

    int a = 10;
    int b = 20;

    @org.junit.Test
    public void show(){
        System.out.println("test中:\na="+a+"  b="+b);
        b += 10;
    }

    @Before
    public void before(){
        a += 5;
        System.out.println("test之前运行");
        System.out.println("a="+a+"  b="+b);

    }

    @After
    public void getint(){
        System.out.println("test之后自动运行");
        System.out.println("a="+a+"  b="+b);
    }
}
```





# JDBC编程

导包mysql-connecter

## 四个对象

DriverManager：驱动

Connection：连接

Statement：执行sql的对象，一般用子类，PreparedStatement

ResultSet：保存结果

executeQuery：执行查询语句

executeUpdate：执行更新语句

**关闭自动提交**：`conn.setAutoCommit(false)`

**开启预编译**：默认关闭，需要在URL后加上`useServerPrepStmts=true`

<div align="left">
    <img src="C:\Users\49584\Desktop\Java\picture\jdbc-resultset.png" style="zoom:67%;" />
</div>



## 基本流程

```java
public class Connect {
    public static void main(String[] args) throws ClassNotFoundException, SQLException {
        // 1. 注册驱动
        Class.forName("com.mysql.cj.jdbc.Driver");
        // 2. 获得连接
        Connection conn=DriverManager.getConnection("jdbc:mysql://localhost:3306/jing_dong","root","root");
        // 3. 获得执行sql的对象
        Statement st = conn.createStatement();
        // 4. 执行sql
        ResultSet rs = st.executeQuery("select * from goods");

        // 4.1 预编译，从1开始
        String  sql = "select * from goods where name=?";
        PreparedStatement st1 = conn.prepareStatement(sql);
        st1.setString(1,"商务双肩背包");
        ResultSet st1 = st1.executeQuery();

        // 打印结果到控制台
        ResultSetMetaData rsmd = rs.getMetaData();
        int columnsNumber = rsmd.getColumnCount();
        while (rs.next()) {
            for (int i = 1; i <= columnsNumber; i++) {
                if (i > 1) System.out.print(",  ");
                String columnValue = rs.getString(i);
                System.out.print(columnValue + " " + rsmd.getColumnName(i));
            }
            System.out.println("");
        }


        // 5. 关闭资源
        st1.close();
        st1.close();
        rs.close();
        st.close();
        conn.close();
    }
}
```

## 事务

```java
Class.forName("com.mysql.cj.jdbc.Driver");
Connection conn = DriverManager.getConnection("jdbc:mysql://localhost:3306/test");


String sql = "update test.t1 set ci_no = 111111 where rec_no=?;";
PreparedStatement statement = conn.prepareStatement(sql);
statement.setString(1,"111111");
try {
    conn.setAutoCommit(false);// 开启事务
    int i = statement.executeUpdate(); // 返回影响行数
    conn.commit();
} catch (SQLException e) {
    conn.rollback();    // 失败回滚
    throw new RuntimeException(e);
}

statement.close();
conn.close();
```

## 连接池

<div align="left">
    <img src="C:\Users\49584\Desktop\Java\picture\jdbc-连接池.png" style="zoom:70%;" />
</div>



```java
阿里巴巴的连接池，常用；
// 1.导入jar
<dependency>
    <groupId>com.alibaba</groupId>
    <artifactId>druid</artifactId>
    <version>1.0.26</version>
</dependency>
// 2.定义配置文件：druid.properties
    driverClassName=com.mysql.cj.jdbc.Driver
    url=jdbc:mysql://localhost:3306/test?useSSl=false&useServerPrepStmts=true
    username=root
    password=root
    # 初始化连接数量
    initialSize=5
    # 最大连接数
    maxActive=10
    # 最大等待时间
    maxWait=3000
// 3.加载配置文件
Properties prop = new Properties();
prop.load(new FileInputStream("src/main/resources/druid.properties"));

// 4.获取连接池对象
DataSource dataSource = DruidDataSourceFactory.createDataSource(prop);
Connection connection = dataSource.getConnection();


************************************************************************************************
public class Test_Conn {
    public static void main(String[] args) throws SQLException {
        Connection conn =  C3P0Utils.getConnection();
        String SQL = "select * from goods";
        PreparedStatement st = conn.prepareStatement(SQL);
        ResultSet res = st.executeQuery();
        ResultSetMetaData md =  res.getMetaData();
        int row_number = md.getColumnCount();
        while (res.next()){
            for(int i=1;i<=row_number;i++){
                System.out.print(res.getString(i)+"   ");
            }
            System.out.println("");
        }
        conn.close();
    }
}
```



# 线程和进程

## 线程

一般新建一个Thread子类，实现run方法，在调用时new 该类，调用start方法

#### **方法1**

```java
public class TestTread {
	//thread 有setName和getName设置线程名称
    public static void main(String[] args) {
        //完整的lambda表达式
        Thread t = new Thread(new Runnable() {
            @Override
            public void run() {
                show1();
            }
        }, "task");// 双::为lambda表达式的实现


        // t1,t2,t3效果一样，简单写法
        Thread t1 = new Thread(() -> {
            show1();
        },"task-name1");
        Thread t2 = new Thread(TestTread::show2, "task-name2");// 双::为lambda表达式的实现
        Thread t3 = new Thread(() -> show1(), "task3");
        t.start();
        t1.start();
        t2.start();
        t3.start();
    }


    public static void show1(){
        for (int i=0;i<1000;i++) {
            System.out.println(Thread.currentThread().getName() +" is runing");
        }
    }

    public static void show2(){
        for (int i=0;i<1000;i++) {
            System.out.println(Thread.currentThread().getName() +" is runing");
        }
    }
}

```

#### 方法二：Runnable接口

优点：可以继承其他类，可扩展性强

```java
// 第一步：线程子类，实现接口
public class MyThread implements Runnable{
    @Override
    public void run() {
        for (int i=0;i<1000;i++) {
            System.out.println(Thread.currentThread().getName()+" is runing");
        }
    }
}
// 第二步：创建子线程类，该类是个接口，不能调用start方法
public class TestTread {
    public static void main(String[] args) {
        MyThread t1 = new MyThread();
        MyThread t2 = new MyThread();
        Thread thread1 = new Thread(t1,"task1");
        Thread thread2 = new Thread(t2,"task2");
        thread1.start();
        thread2.start();
    }
}
```

### 线程冲突案例：卖票

解决方法：锁对象，可为任意对象new Object、Ticket.class、this

 **票的实现类**

```java
public class Ticket implements Runnable{
    private int ticket = 1000;
    @Override
    public void run() {
        synchronized (this){ // 添加锁
            while (ticket>0) {
                System.out.println(Thread.currentThread().getName() + "正在卖第" + ticket + "张票");
                ticket--;
            }
        }
    }
}

```

**三个窗口同时卖票**

```java
public class SellTicket {
    public static void main(String[] args) {
        Ticket t1 = new Ticket();
        Ticket t2 = new Ticket();
        Ticket t3 = new Ticket();
        Thread thread1 = new Thread(t1,"窗口一");
        Thread thread2 = new Thread(t2,"窗口二");
        Thread thread3 = new Thread(t3,"窗口三");
        thread1.start();
        thread2.start();
        thread3.start();
    }
}
```



# maven

编译命令 mvn compile

清除编译：mvn clean

测试：mvn test

将项目打包方法仓库里面：maven install

打包时指定环境：install -P dep_env

打包时跳过测试：install -D skipTests

pom.xml

```xml
<?xml version="1.0" encoding="UTF-8"?>  默认格式
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>  当前maven发展到什么版本了

    这四项和当前项目有关
    <groupId>com.itheima</groupId>  包名，应项目下src的路径
    <artifactId>test-maven</artifactId>  项目名
    <version>1.0-SNAPSHOT</version>  版本;snapshot开发版，release完成版
	<packaging>jar</packaging>  作为干什么用的


	<!--  定义该工程用于进行项目管理
    <packaging>pom</packaging>
    <modules>
        <module>../maven-test/test01</module>
        <module>../maven-test/test02</module>
        <module>../test03</module>
    </modules>
	-->

    <!--  多环境  -->
    <profiles>
    <!-- 生产环境 -->
        <profile>
            <id>pro_env</id>
            <properties>
                <jdbc.url>变量值</jdbc.url>定义变量
            </properties>
            <activation>
            	<activateByDefault>true</activateByDefault>默认启动该环境
            </activation>
        </profile>

    <!-- 开发环境 -->
        <profile>
            <id>pro_env</id>
            <properties>
                <jdbc.url>变量值2</jdbc.url>定义变量
            </properties>
        </profile>
    </profiles>

    <!-- 定义自定义属性 -->
    <properties>
        <maven.compiler.source>17</maven.compiler.source>
        <maven.compiler.target>17</maven.compiler.target>
        <!-- 指定spring的版本，下面版本中间这样写<version>${spring.version}</version>-->
        <spring.version>5.1.9.RELEASE</spring.version>
    </properties>





    <dependencyManagement></dependencyManagement>用这个将dependencies包起来则，子model可以继承
    <dependencies>依赖
        <dependency>
            <groupId>com.google.code.gson</groupId>
            <artifactId>gson</artifactId>
            <version>2.2.4</version>
            <optional>true</optional>//可选依赖，别人用我的jar时，看不见我用了这个
        </dependency>

        添加一个一个依赖，并且排除其中自己不要的依赖
        <dependency>
            <groupId>org.example</groupId>
            <artifactId>test02</artifactId>
            <version>1.0-SNAPSHOT</version>
            <optional>true</optional>
            <scope>test</scope> 限制依赖的使用范围
            <exclusions> 排除
                <exclusion>
                    <groupId>junit</groupId>
                    <artifactId>junit</artifactId>
                </exclusion>
            </exclusions>
        </dependency>
    </dependencies>



    <plugns>
      <plugin>
        <groupId>org.apache.maven.plugins</groupId>
        <artifactId>maven-surefire-plugin</artifactId>
        <version>2.22.1</version>
        <!--<configuration><skipTests>true</skipTests></configuration>跳过全部测试-->
        <configuration>
          <includes>
            <!--  -->
            <include>**/DemoTest.java</include>
          </includes>
          <excludes>
            <exclude></exclude>
          </excludes>
        </configuration>
      </plugin>
	</plugns>

</project>
```

<img src="C:\Users\49584\Desktop\Java\picture\maven手动创建.png" style="zoom:80%;" />

<img src="C:\Users\49584\Desktop\Java\picture\依赖限制使用范围.png" style="zoom:70%;" />

# 日志系统

## log4j

### 三大组件

**Loggers**

> 控制日志的输出级别

**Appenders**

> 指定日志的输出方式，控制台，文件等

| 输出端类型               | 作用                                                         |
| ------------------------ | ------------------------------------------------------------ |
| ConsoleAppender          | 将日志输出到控制台                                           |
| FileAppender             | 将日志输出到文件                                             |
| DailyRollingFileAppender | 将日志输出到一个日志文件，并且每天输出到一个新的文件         |
| RollingFileAppender      | 将日志信息输出到一个日志文件并且指定文件的尺寸，当大小达到指定尺寸时，会自动把文件改名，同时产生一个新的文件 |
| JDBCApender              | 将日志信息保存到数据库中                                     |

**Layout**

> 控制日志的输出格式

| 格式化器类型  | 作用                                                   |
| ------------- | ------------------------------------------------------ |
| HTMLLayout    | 格式化输出为html表格形式                               |
| SimpaleLayout | 简单的日志输出格式化，打印的日志格式为（info-message） |
| PatternLayout | 最强大的格式化器，可以自定义输出日志格式               |



**日志配置文件**

```java
// 所有的配置
static final String CATEGORY_PREFIX = "log4j.category.";
static final String LOGGER_PREFIX = "log4j.logger.";
static final String FACTORY_PREFIX = "log4j.factory";
static final String ADDITIVITY_PREFIX = "log4j.additivity.";
static final String ROOT_CATEGORY_PREFIX = "log4j.rootCategory";
static final String ROOT_LOGGER_PREFIX = "log4j.rootLogger";
static final String APPENDER_PREFIX = "log4j.appender.";
static final String RENDERER_PREFIX = "log4j.renderer.";
static final String THRESHOLD_PREFIX = "log4j.threshold";
private static final String THROWABLE_RENDERER_PREFIX = "log4j.throwableRenderer";
private static final String LOGGER_REF = "logger-ref";
private static final String ROOT_REF = "root-ref";
private static final String APPENDER_REF_TAG = "appender-ref";
public static final String LOGGER_FACTORY_KEY = "log4j.loggerFactory";
private static final String RESET_KEY = "log4j.reset";
private static final String INTERNAL_ROOT_NAME = "root";
```

```properties
log4j.rootLogger=trace,console
log4j.appender.console=org.apache.log4j.ConsoleAppender
# 指定消息格式
#log4j.appender.console.layout=org.apache.log4j.SimpleLayout
#log4j.appender.console.layout=org.apache.log4j.HTMLLayout
#自定义输出规则
log4j.appender.console.layout=org.apache.log4j.PatternLayout
log4j.appender.console.layout.conversionPattern=%r [%t] %p %c %x -%m%n


# 向文件中写
log4j.rootLogger=trace,console,file
log4j.appender.file=org.apache.log4j.FileAppender
# 指定消息格式
#log4j.appender.file.layout=org.apache.log4j.SimpleLayout
#log4j.appender.file.layout=org.apache.log4j.HTMLLayout
#自定义输出规则
log4j.appender.file.layout=org.apache.log4j.PatternLayout
log4j.appender.file.layout.conversionPattern=%r [%t] %p %c %x -%m%n

#指定路径和编码
log4j.appender.file.file=/home/chenjia/log4j.log
log4j.appender.file.encoding=utf-8

# RollingFileAppender需要的,超过1M,创建新的写入，改名旧的，最多10个，然后覆盖最旧的一个
log4j.rootLogger=trace,rollingFile
log4j.appender.rollingFile=org.apache.log4j.RollingFileAppender
log4j.appender.rollingFile.maxFileSize=1MB
log4j.appender.rollingFile.maxBackupIndex=10

# 按时间拆分，每个时间写入一个新的文件
log4j.rootLogger=trace,daliyFile
log4j.appender.dailyFile.layout=org.apache.log4j.PatternLayout
log4j.appender.dailyFile.layout.conversionPattern=%r [%t] %p %c %x -%m%n
log4j.appender.dailyFile.file=./logs/flink.log
log4j.appender.dailyFile.encoding=utf-8
log4j.appender.dailyFile=org.apache.log4j.DailyRollingFileAppender
log4j.appender.dailyFile.datePattern='.'yyyy-MM-dd-HH


# 写入数据库：mysql
log4j.rootLogger.appender=logDB
log4j.appender.logDB=org.apache.log4j.jdbc.JDBCAppender
log4j.appender.logDB.layout=org.apache.log4j.PatternLayout
log4j.appender.logDB.Driver=com.mysql.jdbc.Driver
log4j.appender.logDB.URL=jdbc:mysql://loaclhost:3306/test
log4j.appender.logDB.User=root
log4j.appender.logDB.Password=123456
log4j.appender.logDB.Sql=INSERET INTO #插入语句，需要先在mysql中创建表
```

> **Layout格式**
>
> %m  输出代码中指定的日志信息
>
> %p  日志的优先级
>
> %n  换行符
>
> %r  输出自应用启动到输出该log信息花费的毫秒数
>
> %c  输出打印语句所属类名的全名
>
> %t  输出产生该日志的线程全名
>
> %d  输出服务器当前时间，默认ISO8601，指定格式如：%d{yyyy-MM-dd HH:mm:ss}
>
> %l  输出日志时间发生的位置，包括类名、线程、及所在代码行数：Test.main(Test.java:10)
>
> %F  输出日志消息产生时所在的文件名称
>
> %L  输出代码中的行号
>
> %%  输出一个%号
>
> `可以在%与字符间加上修饰符来控制最小宽度、最大宽度和文本的对齐方式`
>
> %5c        输出类名，最小宽度为5，默认右对齐
>
> %-5c       左对齐，补空格
>
> %.5c       最大宽度为5，右对齐，多余的截取掉
>
> %20.30c    最小为20，少了补空格，最大为三十，多了截取掉



**自定义Logger**

```java
package example;
Logger logger = Logger.getLogger(example.class);

log4j.logger.example=info,file
```

## JCL

**日志门面技术已废弃**

```xml
<dependency>
    <groupId>commons-logging</groupId>
    <artifactId>commons-logging</artifactId>
    <version>1.1.3</version>
</dependency>
```

> 没有引入log4j则将使用，jdk的logger。引入后自动使用log4j

## slf4j

**日志门面技术**

```xml
# 门面依赖
<dependency>
	<groupId>org.slf4j</groupId>
	<artifactId>slf4j-api</artifactId>
	<version>1.7.32</version>
</dependency>

# 门面开关，关闭后日志失效
<dependency>
    <groupId>org.slf4j</groupId>
    <artifactId>slf4j-nop</artifactId>
    <version>1.7.32</version>
</dependency>

# 简单日志实现
<dependency>
	<groupId>org.slf4j</groupId>
	<artifactId>slf4j-simple</artifactId>
	<version>1.7.32</version>
</dependency>

# 绑定logback：常用
<dependency>
    <groupId>ch.qos.logback</groupId>
    <artifactId>logback-classic</artifactId>
    <version>1.4.4</version>
</dependency>

# 绑定log4j
<dependency>
    <groupId>log4j</groupId>
    <artifactId>log4j</artifactId>
    <version>1.2.17</version>
</dependency>
# 适配器
<dependency>
    <groupId>org.slf4j</groupId>
    <artifactId>slf4j-log4j12</artifactId>
    <version>1.7.25</version>
</dependency>


# 绑定jdk14
<dependency>
    <groupId>org.slf4j</groupId>
    <artifactId>slf4j-jdk14</artifactId>
    <version>1.7.36</version>
</dependency>


# 桥接器,不能和桥接器同时存在
<dependency>
    <groupId>org.slf4j</groupId>
    <artifactId>log4j-over-slf4j</artifactId>
    <version>1.7.36</version>
</dependency>
```

slf4j简单实现

```java
public class example {
    // log4j的日志记录器
    public static final Logger LOGGER = LoggerFactory.getLogger(example.class);
    @Test
    public void test(){
        LOGGER.error("error");
        LOGGER.warn("warn");
        LOGGER.info("info");
        LOGGER.debug("debug");
        LOGGER.trace("trace");

        // 使用占位符
        String user = "1";
        String password = "a";
        LOGGER.info("用户{}，密码{}",user,password);

        try {
            int i=1/0;
        }catch (Exception e){
            LOGGER.error("出现异常",e);
        }
    }
}
```

## logBack

- logback-core：其他两个模块的基础
- logback-classic：它是log4j的一个改良版本，同时它完整实现了slf4j API
- logback-access：访问模块与servlet容器集成提供通过http来访问日志的功能

logback将会依次读取下列配置文件

- logback.groovy
- logback.test.xml
- logback.xml
- 都没有则采用默认配置

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<configuration>
    <!--  集中配置，设置变量，使用时${name}  -->
    <property name="pattern" value="[%-5level] %d{yyyy-MM-dd HH:mm:ss.SSS} %c %M %L [%thread] %m"></property>
    <!--
      日志输出格式：
      %-5level
      %d{yyyy-MM-dd HH:mm:ss}日期
      %M 为Method
      %c 类的完整名称
      %L 行号
      %thread 线程名称
      %m 或 msg 为信息
      %n 换行
      -->
    <!--  控制台输出appender的配置  -->
    <appender name="console" class="ch.qos.logback.core.ConsoleAppender">
        <target>System.err</target>
        <encoder class="ch.qos.logback.classic.encoder.PatternLayoutEncoder">
            <pattern>${pattern}</pattern>
        </encoder>
    </appender>


    <!-- 文件输出appender-->
    <property name="log_dir" value="/home/chenjia/logs"></property>
    <appender name="file" class="ch.qos.logback.core.FileAppender">
        <file>${lgo_dir}/logback.log</file>
    </appender>

    # html格式
    <appender name="htmlfile" class="ch.qos.logback.core.FileAppender">
        <file>${lgo_dir}/logback.html</file>
        <!--html消息格式配置-->
        <encoder class="ch.qos.logback.core.encoder.LayoutWrappingEncoder">
            <layout class="ch.qos.logback.classic.html.HTMLLayout">
                #<partten>${pattern}</partten>
                <partten>%-5level%d{yyyy-MM-dd HH:mm:ss.SSS}%c%M%L%thread%m</partten>
            </layout>
        </encoder>
    </appender>


    <!--日志拆分和归档压缩的appender-->
    <appender name="rollfile" class="ch.qos.logback.core.rolling.RollingFileAppender">
        <file>${lgo_dir}/roll_logback.log</file>
        <encoder class="ch.qos.logback.classic.encoder.PatternLayoutEncoder">
            <partten>${pattern}</partten>
        </endoder>
        <!--指定拆分规则-->
        <rollingPolicy class="ch.qos.logback.core.rolling.SizeAndTimeBasedRollingPolicy">
            <!--按时间和文件大小拆分-->
            <fileNamePattern>${log_dir}/rolling.%d{yyyy-MM-dd-HH-mm-ss}.log%i.gz</fileNamePattern>
            <maxFileSize>1MB</maxFileSize>
        </rollingPolicy>
                <!--日志级别过滤器-->
        <filter class="ch.qos.logback.classic.filter.LevelFilter">
            <lever>ERROR</lever>
            <onMatch>ACCEPT</onMatch>
            <onMismatch>DENY</onMismatch>
        </filter>
    </appender>

    <!--异步日志-->
    <appender name="async" class="ch.qos.logback.classic.AsyncAppender">
        <appender-ref ref="rollfile"/>
    </appender>

    <!--自定义logger-->
    <logger name="com.example" level="info" additivity="false">
        <appender-ref ref="console"/>
    </logger>
    <!--  配置root logger  -->
    <root level="ALL">
        <appender-ref ref="console"/>
        <appender-ref ref = "file"/>
        <appender-ref ref = "rollfile"/>
        <appender-ref ref = "async"/>
    </root>



</configuration>
```



## log4j2

log4j2+slf4j

```xml
<dependency>
    <groupId>org.slf4j</groupId>
    <artifactId>slf4j-api</artifactId>
    <version>1.7.32</version>
</dependency>

<dependency>
    <groupId>org.apache.logging.log4j</groupId>
    <artifactId>log4j-slf4j-impl</artifactId>
    <version>2.17.1</version>
</dependency>

<!--log4j2的门面实现-->
<dependency>
    <groupId>org.apache.logging.log4j</groupId>
    <artifactId>log4j-api</artifactId>
    <version>2.17.1</version>
</dependency>
<!--log4j2 日志实现-->
<dependency>
    <groupId>org.apache.logging.log4j</groupId>
    <artifactId>log4j-core</artifactId>
    <version>2.17.1</version>
</dependency>

# 使用slf4j绑定log4j2

```

**log4j2.xml文件**

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<Configuration status="warn" monitorInterval="5">
<!--  集中配置，设置变量，使用时${name}  -->
<Properties name="LOG_HOME">/logs</Properties>

<!--  appender的配置  -->
<Appenders name="console" class="ch.qos.logback.core.ConsoleAppender">
    <!--控制台输出-->
    <Console name="Console" target="SYSTEM_ERR">
        <PatternLayout pattern="%d{HH:mm:ss.SSS} [%t] [%-5level] %c{36}:%L --- %m%n"/>
    </Console>

    <!--保存文件-->
    <File name="file" filename="${LOG_HOME}/myfile.log">
        <PatternLayout pattern="%d{HH:mm:ss.SSS} [%t] [%-5level] %c{36}:%L --- %m%n"/>
    </File>

    <!--和文件Appender一样，但是提高了效率-->
    <RandomAccessFile name="accessFile" fileName="%{LOG_HOME}/myAcclog.log">
        <PatternLayout pattern="%d{HH:mm:ss.SSS} [%t] [%-5level] %c{36}:%L --- %m%n"/>
    </RandomAccessFile>


    <RollingFile name="rollingFile" fileName="%{LOG_HOME}/myrolllog.log"
    filePattern="/logs/$${data:yyyy-MM-dd}/myrolling-d{yyyy-MM-dd HH:mm:ss-%i.log}">
        <!--日志级别过滤器-->
        <ThresholdFilter level="debug" onMatch="ACCEPT" onMismatch="DENY"/>
        <PatternLayout pattern="%d{HH:mm:ss.SSS} [%t] [%-5level] %c{36}:%L --- %m%n"/>
        <Policies>
            <!--日志拆分的规则-->
            <!--在系统启动时触发拆分规则，产生一个新的-->
            <OnStartupTriggeringPolicy/>
            <!--文件大小，到达指定大小拆分-->
            <SizeBasedTriggeringPolicy size="10 MB"/>
            <!--按照时间，上面定义的-->
            <TimeBasedTriggeringPolicy/>
        </Policies>
        <!--当前文件夹最多30个，大于30个时，覆盖最旧的-->
        <DefaultRolloverStrategy max="30"/>
    </RollingFile>

</Appenders>


<!--  配置root logger  -->
<Loggers>
    <Root level="trace">
        <appender-ref ref="console"/>
    </Root>
    <!--局部Logger，让该包下的日志单独异步，
	includeLocation="false"关闭日志记录的行号信息，这个大幅提升性能
 	additivity="false"不继承rootLogger对象-->
    <AsyncLogger name="com.example" level="warn" includeLocation="false" additivity="false">
        <AppenderRef ref="Console"/>
    </AsyncLogger>
</Loggers>
</Configuration>
```

### 全局异步方式

**异步Appender**

```xml
<!--保存文件-->
<File name="file" filename="${LOG_HOME}/myfile.log">
    <PatternLayout pattern="%d{HH:mm:ss.SSS} [%t] [%-5level] %c{36}:%L --- %m%n"/>
</File>
<!--给文件输出设置成异步-->
<Async name="Async">
    <AppenderRef ref="file"/>
</Async>
```



**异步logger**

无序修改log4j2.xml文件，新建一个log4j2.component.properties，加入一个配置

```properties
Log4jContextSelector=org.apache.logging.log4j.core.async.AsyncLoggerContextSelector
```

### 局部异步方式

**异步Logger**$\color{#ff0000}{常用}$

```xml
<!--局部Logger，让该包下的日志单独异步，
 includeLocation="false"关闭日志记录的行号信息，这个大幅提升性能
  additivity="false"不继承rootLogger对象-->
<AsyncLogger name="com.example" level="warn" includeLocation="false" additivity="false">
    <AppenderRef ref="Console"/>
</AsyncLogger>
```

## 无垃圾模式

缓冲区+重用对象
