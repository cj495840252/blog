# kafka



## 整体架构

**去中心化**

两种模式：

- 点对点：一个生产者，一个消费者，一个topic； 消费后就删除数据
- 发布订阅：多个生产者，多个消费者，相互独立，多个topic ，不会删除数据

架构：

- 生产者：生产数据

  1. 实例化生产者
  2. send(,callback)方法
  3. 拦截器
  4. 序列化器
  5. 分区器
  6. sender线程：分区完成后批次拉取数，由linger.ms和批次大小决定
  7. sender线程最多缓存5个
  8. selector，打通io流，发送数据
  9. broker返回ack校验

- broker

  1. broker 服务器
  2. topic 主题，对于数据分类
  3. partitions分区，解决数据量大，处理不过来，也可以让多个消费者读取
  4. 副本：一个数据多个存储，增加可靠性
  5. leader，follower：存储数据
     - leader一般均匀分布在broker上，producer和consumer操作leader partition
  6. 消费者

  1. 消费者和消费者独立

  2. 消费者组

     某一个分区，只能由一个消费者消费

- zookeeper

  1. 存储broker.ids ：0 1 2
  2. 每个topic下的每个分区对应的leader和ISR

broker的配置参数

- batch.size:批次大小
- linger.ms: 等待时间，修改为5-100ms
- compression.type: 压缩snappy
- RecordAccumulator: 缓冲区大小，修改为64

1. **可以提高生产者的吞吐量**

## 常用命令行

- 主题 kafka-topics.sh
  1. --bootstrap-server  node1:9092,node2:9092,node3:9092
  2. --topic first
  3. --create
  4. --delete
  5. --alter
  6. --list
  7. --describe
  8. --partitions
  9. --repliation-factor 副本存放因子

- 生产者 kafka-console-producer.sh

  1. --bootstrap-server  node1:9092,node2:9092,node3:9092
  2. --topic test

- 消费者 kafka-console-consumer.sh

  1. --bootstrap-server  node1:9092,node2:9092,node3:9092
  2. --topic test

- 查看data下log文件

  ```shell
  kafka-run-class.sh kafka.tools.DumpLogSegments --files ./000.index
  ```

- 查看kafka内存使用情况

  ```shell
  jps : 查看进程号
  jstat -gc kafka进程号 ls 10
  jmap -heap kafka进程号
  ```

- 集群压力测试

  **生产者压力测试**

  ```shell
  kafka-producer-perf-test.sh --topic test --record-size 1024 --num-records 1000000 --throughput 10000 --producer-props bootstrap.servers=node1:9092,node2:9092,node3:9092 batch.size=16384 linger.ms=0 compression.type=snappy buffer.memory=67108864

  --record-size # 一条信息有多大，单位字节，本次测试1k
  --num-records # 总共发送多少条信息，本次测试设置100w条
  --throughput  # 每秒多少条信息，-1表示不限流，测出生产者最大吞吐量,本次测试每秒1w条
  --producer-props # 可以配置生产者相关配置

  # 下列四个参数可以提高整体的吞吐量
  batch.size=16384
  linger.ms=0
  compression.type=snappy
  buffer.memory=67108864
  ```

  **消费者压力测试**

  ```shell
  # 配置信息需要指定配置文件
  kafka-consumer-perf-test.sh \
  --broker- node1:9092,node2:9092,node3:9092 \
  --topic test \
  --message 1000000 \
  --consumer.config config/consumer.properties

  max.poll.records=500 # 一次拉取条数
  ```



## 代码示例

生产者

```java
public class KfkProdecer {
    public static void main(String[] args) {
        Properties props = new Properties();
        props.put("bootstrap.servers","192.168.88.161:9092");
        props.put("ackks","all");
        props.put("key.serializer","org.apache.kafka.common.serialization.StringSerializer");
        props.put("value.serializer", "org.apache.kafka.common.serialization.StringSerializer");

        KafkaProducer<String, String> producer = new KafkaProducer<>(props);


        for (int i = 0; i < 100; i++) {
            // 同步方式
            // Future<RecordMetadata> future = producer.send(new ProducerRecord<String, String>("test", null, i + ""));
            // future.get();
            // 异步
            producer.send(new ProducerRecord<String, String>("test", Integer.toString(i), i + ""), new Callback() {
                @Override
                public void onCompletion(RecordMetadata recordMetadata, Exception e) {
                    if (e != null) System.out.println("发送消息出现异常");
                    else {
                        String topic = recordMetadata.topic();
                        int partition = recordMetadata.partition();
                        long offset = recordMetadata.offset();
                        System.out.println("发送消息到kafka中的名字为"+topic+"的主题，第"+partition+"分区，第"+offset+"条数据成功！");
                    }
                }
            });
        }

        producer.close();
    }
}
```

消费者

```java
public class kfkCustomer {
    public static void main(String[] args) {
        Properties props = new Properties();
        props.setProperty("bootstrap.servers", "node1.itcast.cn:9092");
        props.setProperty("group.id", "test");
        props.setProperty("enable.auto.commit", "true");
        props.setProperty("auto.commit.interval.ms", "1000");
        props.setProperty("key.deserializer", "org.apache.kafka.common.serialization.StringDeserializer");
        props.setProperty("value.deserializer", "org.apache.kafka.common.serialization.StringDeserializer");

        // 2. 创建一个消费者
        KafkaConsumer<String, String> consumer = new KafkaConsumer<>(props);
        consumer.subscribe(Arrays.asList("test"));
        while (true){
            ConsumerRecords<String, String> records = consumer.poll(Duration.ofMillis(100));
            for (ConsumerRecord<String, String> record : records) {
                System.out.printf("offset=%d，partitions=%s, key=%s,
                                  value=%s%n",record.offset(),record.partition(),record.key(),record.value());
            }
        }
    }

}
```



## 生产者

传入数据时。按key：value

<img src="C:\Users\49584\Desktop\Java\picture\生产者发送数据的流程.png" style="zoom:60%;" />

### 吞吐量的提高

1. 批次大小增大，默认16k，增加到32

2. linger.ms   默认0 ，==>5-100ms  双刃剑
3. 压缩
4. 缓存大小

```java
// Properties继承于hashtable，可以使用put方法
// Properties提供了setProperty方法进行参数配置，不应该使用put方法
Properties props = new Properties();// 创建配置类

// 建立连接，两个一样的效果
props.put("bootstrap.servers", "node1.itcast.cn:9092");
props.put(ProducerConfig.BOOTSTRAP_SERVERS_CONFIG,"node1:90922");


props.put("transactional.id", "dwd_user");
// 序列化key 和value，两种方式
props.put("key.serializer", "org.apache.kafka.common.serialization.StringSerializer");
props.put("value.serializer", "org.apache.kafka.common.serialization.StringSerializer");
props.put(ProducerConfig.KEY_SERIALIZER_CLASS_CONFIG, StringSerializer.class.getName());
props.put(ProducerConfig.VALUE_SERIALIZER_CLASS_CONFIG, StringSerializer.class.getName());

// 缓冲区大小
props.put(ProducerConfig.BUFFER_MEMORY_CONFIG,33554432);// 32m


// 批次大小
props.put(ProducerConfig.BATCH_SIZE_CONFIG,16384);//16k


// linger.ms
props.put(ProducerConfig.LINGER_MS_CONFIG,1);

// 压缩格式
props.put(ProducerConfig.COMPRESSION_TYPE_CONFIG,"snappy");

//
props.put("transactional.id", "dwd_user");

// ack级别
props.put(ProducerConfig.ACKS_CONFIG,"-1");
props.put("ackks","all");// all和-1是相等的

// 重试次数
props.put(ProducerConfig.RETRIES_CONFIG,3);//默认int最大值

// 幂等性设置
props.put("enable.idempotence",false);
props.put(ProducerConfig.ENABLE_IDEMPOTENCE_CONFIG,true);

// 自定义事务id
props.put("transactional.id", "dwd_user");
props.put(ProducerConfig.TRANSACTIONAL_ID_CONFIG,"dwd_user");

// 分区器
props.put(ProducerConfig.PARTITIONER_CLASS_CONFIG,"org.example.MyPartitioner");
```

### 写入分区的策略

- 轮询分区策略：用的最多，默认，数据乱序
- 随机分区策略：数据乱序，不指定key时，粘性，直到时间到达或者分区存满为止选择下一个，和这个不相同
- 按key分区：可能数据倾斜，但是可以保持一定的顺序
- 自定义分区：

生产中将一个表名作为key值，可以将一张表发到一个分区

```java
partiton方法的参数
String topic：主题;
Object key: 传递的key值;
Object value:传递的value值;
byte[] bytes:序列化之后的key值;
byte[] bytes1:序列化之后的value值;

// 将自定义的分区器关联到配置中
props.put(ProducerConfig.PARTITIONER_CLASS_CONFIG,"org.example.MyPartitioner");
props.put(ProducerConfig.PARTITIONER_CLASS_CONFIG, MyPartitioner.class.getName());
//自定义分区器，这里实现随机分区
public class MyPartitioner implements Partitioner{
    private Random r;
    @Override
    public int partition(String topic, Object key, byte[] bytes, Object value, byte[] bytes1, Cluster cluster) {
        return r.nextInt(1000)%cluster.partitionCountForTopic(topic);
    }

    @Override
    public void close() {

    }

    @Override
    public void configure(Map<String, ?> map) {
        Random r = new Random();
    }
}
```

### ACK应答

- 0：生产者发送数据后就不管了，可靠性差效率高
- 1 ：leader收到生产者的数据后未同步follower挂掉，选举新的leader找不到数据，而丢失数据
- -1：leader和ISR队列中所有follower应答，可靠性高，效率低，低概率造成数据重复

-1是最高的，生产者发生数据后，leader接收数据，follower拉取leader数据，给leader反馈拉取完毕，leader接收所有反馈后，反馈ack给生产者。leader维护了一个动态ISA队列，若follower长时间未向leader发送通宵请求或者同步数据，该follower将会被剔除ISA队列

数据完全可靠的条件：ACK级别设置-1 + 分区副本大于等于2 + ISA中应答的最小副本数>=2

ISA中最小副本数量：min.insync.replicas默认为1，和ack为1是一样的

> 生产中，ack=0，很少使用；ack=1，一般用于传输普通日志数据，ack=-1，一般用于传输和钱相关的对可靠性高的场景

**的数据重复问题**

原因：ACK = -1时，leader在同步快完成时，leader挂了，新的leader重新同步一遍

### 数据乱序

默认每个分区缓存5个请求

- kafka在1.x之前保证单分区有序

  ```java
  // 请求缓存改为1
  max.in.flight.requests.per.connectio=1
  ```

- 1.x之后保证单分区有序

  - 未开启幂等性：`max.in.flight.requests.per.connectio` = 1
  - 开启幂等性：`max.in.flight.requests.per.connectio` <= 5即可



### 幂等性

```java
// 幂等性设置
props.put("enable.idempotence",false);
props.put(ProducerConfig.ENABLE_IDEMPOTENCE_CONFIG,true);
```

不论Producer向Broker发送多少重复数据 ，Broker端都只会持久化一条数据

精确一次发送：幂等性+至少一次（ack=-1 + 分区副本数>=2 + ISA中最少应答副本数>=2）

**数据重复的判断标准**

PID：kafka的进程号，每次重启都会分配一个新的

Partiton: 分区号

SeqNumber: 单调递增的

**幂等性只能保证单会话、单分区内不重复**

### 事务

每个broker中都有事务协调器（Transaction Coordinator），由一个存储事务消息的特殊主题来确定使用哪一个事务协调器。事务的划分是根据transactional.id的hashcode%分区数来确定使用leader中哪个broker的事务协调器

Producer使用事务前，需要先自定义事务id

```java
// 自定义事务id,名字随便起，单要保证唯一
props.put("transactional.id", "dwd_user");
props.put(ProducerConfig.TRANSACTIONAL_ID_CONFIG,"dwd_user");

// 1 初始化事务
producer.initTransactions();
// 2 开启事务
producer.beginTransaction();
// 3 在事务内提交已经消费的偏移量 （消费者）
producer.sendOffsetsToTransaction(offsetCommit,"ods_user");
// 4 提交事务
producer.commitTransaction();
// 5 放弃事务
producer.abortTransaction();

public static void main(String[] args) {
        Consumer<String,String> consumer = TransformConsumer.createConsumer();
        Producer<String,String> producer = TransformProducer.createProducer();

        // 1.初始化事务表
        producer.initTransactions();

        while (true){
            try {
                // 2.开启事务
                producer.beginTransaction();
                Map<TopicPartition, OffsetAndMetadata> offsetCommit =new HashMap<>();
                ConsumerRecords<String ,String> records = consumer.poll(Duration.ofSeconds(2));
                for (ConsumerRecord<String,String> record:records){
                    offsetCommit.put(
                        new TopicPartition(record.topic(), record.partition()),new OffsetAndMetadata(record.offset()+1)
                    );
                    String[] fields = record.value().split(",");
                    fields[1] = fields[1].equalsIgnoreCase("1")?"男":"女";
                    String message = fields[0] + "," + fields[1] + fields[2];

                    // 模拟异常
                    // int i = 1/0;
                    //生产消息到dwd_user
                    producer.send(new ProducerRecord<>("dwd_user",message));
                }
                // 3.提交偏移量到事务
                producer.sendOffsetsToTransaction(offsetCommit,"ods_user");

                // 4.提交事务
                producer.commitTransaction();


            }catch (Exception e){
                // 5. 终止事务
                producer.abortTransaction();
            }
            finally {
                // 关闭生产者
                producer.close();
            }
        }
    }
```



## 消费者

```java
Properties properties = new Properties();
// 连接kafka
properties.setProperty(ConsumerConfig.BOOTSTRAP_SERVERS_CONFIG,"node1:9092,node2:9092");


// 反序列化
properties.setProperty(ConsumerConfig.KEY_DESERIALIZER_CLASS_CONFIG, StringDeserializer.class.getName());
properties.setProperty(ConsumerConfig.VALUE_DESERIALIZER_CLASS_CONFIG,StringDeserializer.class.getName());
props.setProperty("key.deserializer","org.apache.kafka.common.serialization.StringDeserializer");
props.setProperty("value.deserializer","org.apache.kafka.common.serialization.StringDeserializer");

properties.setProperty("auto.offset.reset","earliest");

// 消费者组id
properties.setProperty(ConsumerConfig.GROUP_ID_CONFIG,"test_group_id");
props.setProperty("group.id", "test");

// 分区分配策略
properties.psetProperty(ConsumerConfig.partition_assignment_strategy_config,"org.apache.clients.consumer.RoundRobin");

// 自动提交offset，默认true
properties.setProperty("enable.auto.commit","true");
// 自动提交的时间间隔，默认5s
properties.setProperty("auto.commit.interval.ms","5s");
```



### 吞吐量的提高

- 数据积压

1. 增加topic的分区数和consumer个数
1. 提高每批次的拉取数量



### 消费者组

- 消费者组：由多个consumer组成

- 每个consumer分别复制消费不同分区的数据，一个分区只能由一个组内消费者消费

- 消费者组之间互不影响

- 组内消费者大于分区数时，多余的消费者空闲不工作

- coordinator:辅助实现消费者组的初始化和分区的分配，在每个broker上都有一个coordinator

  coordinator节点选择 = groupid的hashcode%50（50是_consumer_offsets的分区数量）

  <div>
      <img src="C:\Users\49584\Desktop\Java\picture\消费者组的初始化.png" style="zoom:50%;" />
  </div>

### 再平衡

- session.timeout.ms = 45s

  每个消费者都会和coordinator保持心跳，默认3s，超过设置的45s，将会移除该消费者，触发再平衡

- max.poll.interval.ms = 5分钟

  消费者处理消息的时间过长，超过设置的5分钟，也会触发再平衡

### 消费流程

- ConsumerNetworkClient：连接broker的app
- Fetch.min.bytes：每批次最小抓取大小，默认1字节
- fetch.max.wait.ms：抓取一批数据最小值未达到的超时时间，500ms
- fetch.max.bytes：每批次最大抓取大小，默认50M
- max.poll.records：一次抓取数据返回消息的最大条数，默认500条

<div>
    <img src="C:\Users\49584\Desktop\Java\picture\消费流程.png" style="zoom:50%;" />
</div>

### 指定消费分区

```java
Properties properties = new Properties();
// 连接kafka
properties.put(ConsumerConfig.BOOTSTRAP_SERVERS_CONFIG,"node1:9092,node2:9092");
properties.put(ConsumerConfig.GROUP_ID_CONFIG,"test_group_id");

// 自动提交
props.setProperty("enable.auto.commit", "true");
props.setProperty("auto.commit.interval.ms", "1000");

// 反序列化
properties.put(ConsumerConfig.KEY_DESERIALIZER_CLASS_CONFIG, StringDeserializer.class.getName());
properties.put(ConsumerConfig.VALUE_DESERIALIZER_CLASS_CONFIG,StringDeserializer.class.getName());
KafkaConsumer<String, String> consumer = new KafkaConsumer<String, String>(properties);

// 订阅
consumer.subscribe(List.of("test"));//订阅主题
consumer.assign(List.of(new TopicPartition("test",0)));//订阅主题下的分区


while (true){
    // Duration.ofSeconds(1):批次间隔时间
    ConsumerRecords<String, String> records = consumer.poll(Duration.ofSeconds(1));
    for (ConsumerRecord<String, String> record : records) {
        System.out.println(record);
    }
    // 需要先关闭自动提交
    // consumer.commitSync();// 同步提交
    // consumer.commitAsync(); // 异步提交
}
```



### 分区分配及平衡

- partition.assignment.strategy

  四种主流分配策略：Range，RoundRobin，Sticky，CooperativeSticky

  默认：Range+CooperativeSticky

**Range**

1. 同一个topic的分区按序号排序，消费者按字母顺序排序
2. 每个消费者消费 = `partitions数/consumer数` 个分区,除不尽则前面余数个分区多消费一个
3. 若订阅多个主题，该消费者每个主题都多消费一个，则容易产生数据倾斜

- 某分区挂掉后的45秒内。生产者发送数据，该分区的任务将会给到一个分区

  45s后触发再平衡，重新分配任务

**RoundRobin**

1. 轮询分配策略

2. 将$\color{#ff0000}{所有topic}$的partition和consumer列出来

3. 都按照hashcode排序

4. 轮询算法，分配partition给consumer

5. 某分区挂掉后，45s内将该消费者负责的分区轮询分配给其他分区

   45s后重分区

### offset

查看默认主题中记录的消费者的offset

```shell
/export/server/kafka/bin/kafka-console-consumer.sh \
--topic __consumer_offsets \
--bootstrap-server node1:9092 \
--consumer.config /export/server/kafka/config/consumer.properties \
--formatter "kafka.coordinator.group.GroupMetadataManager\$OffsetsMessageFormatter" \
--from-beginning
```

**手动提交**

- commitSync同步提交：必须等待offset提交完毕，再去消费下一批数据
- commitAsync异步提交：发送完提交offset请求后，就开始消费下一批数据

#### **指定offset消费**

- 订阅主题后加上下面代码

```java
consumer.subscribe(List.of("test"));

// 指定位置消费
Set<TopicPartition> assignment = consumer.assignment(); //该消费者分配的分区的集合
// 保证获取到分区信息
while (assignment.size()==0){  //一直循环到组分配好消费方案
    consumer.poll(Duration.ofSeconds(1));// 拉取值
    assignment = consumer.assignment();//更新分区信息
}
for (TopicPartition partition : assignment) {
    consumer.seek(partition,100);
}
// 改变一个分区
consumer.seek(new TopicPartition("test",0),100);
```

#### **$\color{#ff0000}{指定时间消费}$**

- 订阅主题后加上下面代码

```java
// 指定时间消费
// 获取分区，然后遍历
Set<TopicPartition> assignment = consumer.assignment();
Map<TopicPartition, OffsetAndTimestamp> offsets = consumer.offsetsForTimes(hashMap);
for (TopicPartition partition : assignment) {
    OffsetAndTimestamp offset = offsets.get(partition);
    consumer.seek(partition,offset.offset());
}
// 根据时间获取相关offset
HashMap<TopicPartition, Long> hashMap = new HashMap<>();
for (TopicPartition partition : assignment) {
    hashMap.put(partition,System.currentTimeMillis()-24*3600*1000);
}
Map<TopicPartition, OffsetAndTimestamp> offsets = consumer.offsetsForTimes(hashMap);

// 改变分区中的offset
for (TopicPartition partition : assignment) {
    OffsetAndTimestamp offset = offsets.get(partition);
    consumer.seek(partition,offset.offset());
}
```

### 事务

#### 重复消费和漏消费

- 重复消费：自动提交offset引起，提交后，数据处理了一部分，挂掉。再上线时从提交时重新处理，导致重复消费
- 漏消费：手痛提交时，commit后，数据还在内存中未写入磁盘，进程挂掉，导致该部分数据丢失



## Broker

```properties
log.index.interval.bytes=4kb  采用稀疏索引，默认4kb一个索引
```



### zookeeper存储的kafka信息

- /kafka/brokers/ids [0,1,2] 记录有哪些服务器

- /kafka/brokers/topic/first/partitions/0/state

  {"leader":1,  "isr":[1,0,2]} 记录谁是leader，有哪些服务器可用

- /kafka/controller :  {"broker"  : 0} 辅助选举leader，由它注册和上传节点信息

leader的选举机制

broker在kafka中注册，谁先谁是leader

leader挂掉后，按照AR的顺序选取，ISR中存活的

- AR注册顺序

<img src="C:\Users\49584\Desktop\Java\picture\kafka-broker-zookeeper选举流程.png" style="zoom:75%;" />

### 节点服役和退役

**服役**

test主题：broker_id list：0 1 2  ，现在集群新增一台机器：broker_id = 3

将数据负载的第三台机器上

```json
{	// 要负载哪些主题
    "topics":[
        {"topic":"first_topic_name"},
        {"topic":"second_topic_name"}
        ...
    ],
    "version": 1 // 固定写法
}

//执行命令
bin/kafka-reassign-partitions.sh --bootstrap-server node1:9092 --topics-to-move-json-file 上面的json文件 --broker-list "0,1,2,3...broker_ids" --gengerate

这里会返回一个副本存储计划;
创建新的json文件，将该存储计划放进去

执行该存储计划
bin/kafka-reassign-partitions.sh --bootstrap-server node1:9092 --reassignment-json-file 存储计划.json --execute

验证存储计划
bin/kafka-reassign-partitions.sh --bootstrap-server node1:9092 --reassignment-json-file 存储计划.json --verify
```

**退役**

$\color{#ff0000}{按照服役的方式，减少broker\_id即可}$

```json
{	// 要负载哪些主题
    "topics":[
        {"topic":"first_topic_name"},
        {"topic":"second_topic_name"}
        ...
    ],
    "version": 1 // 固定写法
}
//执行命令
bin/kafka-reassign-partitions.sh --bootstrap-server node1:9092 --topics-to-move-json-file 上面的json文件 --broker-list "0,1,2" --gengerate

这里会返回一个副本存储计划;
创建新的json文件，将该存储计划放进去

执行该存储计划
bin/kafka-reassign-partitions.sh --bootstrap-server node1:9092 --reassignment-json-file 存储计划.json --execute

验证存储计划
bin/kafka-reassign-partitions.sh --bootstrap-server node1:9092 --reassignment-json-file 存储计划.json --verify
```

### follower故障

- LEO：每个副本的最后一个offset，及最新offset+1
- HW：所有副本中最小的LEO

1. follower被踢出ISR
2. leader和其他follower继续接收数据
3. 该follower恢复后，follower会读取本地磁盘记录的上传HW，并将log文件高于HW的部分截取掉，从HW开始向leader同步

### leader故障

1. leader故障后，从ISR中选出新的Leader
2. 为保证数据一致性，其余的follower会将各自的log文件高于HW的部分截掉，然后从新的leader同步数据

- 这只能保证副本之间的数据一致性，并不能保证数据不丢失或者不重复

### 手动调整副本存储

- 创建主题

  ```shell
  # 创建
  kafka-topics.sh --bootstrap-server node1:9092 --create --topic test --partitions 4 --replication-factor 2

  # 查看分布情况
  kafka-topics.sh --bootstrap-server node1:9092 --describe --topic test
  ```

- 创建副本存储计划

  ```json
  {
      "version":1,
      "partitons":[
          {"topic":"test","partition":0,"replications":[0,1]},//test主题，分区为0的副本存放在broker0和broker1上
          {"topic":"test","partition":1,"replications":[0,1]},
          {"topic":"test","partition":2,"replications":[1,0]},
          {"topic":"test","partition":3,"replications":[1,0]},
      ]
  }
  ```



- 执行副本存储计划

  将所有的数据存储在node1 和node2上

  ```shell
  # 执行
  bin/kafka-reassign-partitions.sh --bootstrap-server node1:9092 --reassignment-json-file 存储计划.json --execute

  # 验证存储计划
  bin/kafka-reassign-partitions.sh --bootstrap-server node1:9092 --reassignment-json-file 存储计划.json --verify
  ```

### 增加主题partitions副本数量

```json
// 步骤1 ：创建一个主题
四个分区一个副本
bin/kafka-reassign-partitions.sh --bootstrap-server node1:9092 --create --tpoic test --partitions 4 --replication-factor 1

// 步骤2 ：配置副本存储json文件
增加一个副本
{
    "version":1,
    "partitons":[
        {"topic":"test","partition":0,"replications":[0,1]},//test主题，分区为0的副本存放在broker0和broker1上
        {"topic":"test","partition":1,"replications":[0,1]},
        {"topic":"test","partition":2,"replications":[1,0]},
        {"topic":"test","partition":3,"replications":[1,0]},
    ]
}

// 步骤3 ：执行副本存储
bin/kafka-reassign-partitions.sh --bootstrap-server node1:9092 --reassignment-json-file 存储计划.json --execute
```

### 文件的清除策略

segment:多个大小相等的segment file (段)组成了一个partition。

- kafka的日志的默认保存时间为7天
  - log.retention.hours : 最低优先级，默认7天
  - log.retention.minutes : 分钟
  - log.retention.ms : 最高优先级ms
  - log.retention.check.interval.ms : 检查周期，默认5分钟

- 过期日志删除

  log.cheanup.policy = delete 所有应用启用删除策略

  - 基于时间：默认打开，以segment中所有记录中的最大时间戳作为该文件的时间戳（保证全部超期了才删除）
  - 基于大小：默认关闭。超过设置的所有日志总大小，删除最早的segment
    - log.retention.bytes 默认-1，表示无情的

- 日志压缩

  压缩时，相同的key不同的value保留最新的value

  - log.cleanup = compact 所有数据启用

<div align=left><img src="C:\Users\49584\Desktop\Java\picture\kafka日志文件压缩.png" style="zoom:50%;" /></div>



## kafka如何高效读写数据($\color{#ff0000}{面试}$)

- kafka本身是分布式集群，可以采用分区技术，并行度高

- 所有对数据的操作都在生产者和消费者上

- 读数据采用稀疏索引，可以快速定位要消费的数据

- 顺序写磁盘，一直追加到磁盘

- 页缓存+零拷贝

  - 页缓存：Linux系统提供PageCahe，实际上是尽可能的将空闲内存当磁盘缓存来用

  - 零拷贝：消费者请求数据后，页缓存中的数据直接通过网卡发送

    <div>
        <img src="C:\Users\49584\Desktop\Java\picture\页缓存.png" style="zoom:50%;" />
    </div>



# Kraft模式

移除zookeeper

# Flink与Kafka

## Flink作为生产者

- kafka创建生产者读取flink来的消息，然后发送给主题

```
// 1.获取环境
StreamExecutionEnvironment flink = StreamExecutionEnvironment.getExecutionEnvironment();
flink.setParallelism(3);

// 2.准备数据源
ArrayList<String> list = new ArrayList<>();
list.add("message1");
list.add("message2");
DataStreamSource<String> stream = flink.fromCollection(list);

// 2.1 创建kafka生产者
Properties properties = new Properties();
properties.setProperty(ProducerConfig.BOOTSTRAP_SERVERS_CONFIG,"node1:9092,node2:9092,node3:9092");
// SimpleStringSchema 简单创建了，key、value的序列化方式
FlinkKafkaProducer kafkaProducer = new FlinkKafkaProducer("test",new SimpleStringSchema(),properties);

// 3.添加数据源,addsink保存数据到其他数据库中
stream.addSink(kafkaProducer);
// 4.执行代码
flink.execute();
```



## flink作为消费者

```java
// 1.获取环境
StreamExecutionEnvironment env = StreamExecutionEnvironment.getExecutionEnvironment();
env.setParallelism(3);

// 2.创建一个消费者
Properties properties = new Properties();
properties.setProperty(ConsumerConfig.BOOTSTRAP_SERVERS_CONFIG,"node1:9092,node2:9092");
properties.setProperty(ConsumerConfig.GROUP_ID_CONFIG,"test");
FlinkKafkaConsumer consumer = new FlinkKafkaConsumer<>("test",new SimpleStringSchema(),properties);


// 3.关联消费者和flink
env.addSource(consumer).print();


// 4.执行
env.execute();
```



# kafka调优

## 硬件选择

**生产环境**

- 日志产生情况：100w日活 * 没人每天产生日志100条=1亿条（中型公司）

- 处理日志速度：  一亿条/（24*3600s） = 1150条/s

- 1条日志的大小：（0.5 - 2k 标准1k）

- 平均处理多少大小数据：1150条*1k/s = 1M/s

- 峰值吞吐量：高峰值：1m/s*20倍 = 20m/s - 40m/s

**购买多少台服务器**

服务器台数 = 2*（生产者峰值生产速率 x 副本数  / 100 ）+1

​                    = 2 x (20m/s  x  2/100) + 1

​					= 3 台 （一般公司三台就够了）

**磁盘选择**

机械硬盘和固态硬盘顺序读写速率差不多，所以选择机械硬盘

1亿条 * 1k = 100G

100G * 2个副本 *3 天 / 0.7 = 1T

三台服务器总的磁盘大小 大于1t

**内存选择**

kafka 内存 = 堆内存（kafka内部配置）+ 页缓存（服务器内存）

- 堆内存：10 - 15 G
- 页缓存：segment（1G）  分区数 *1G *25%  / 服务器数（3）= 1G

每台服务器 10G + 1G

**CPU选择**

num.io.threads = 8 ：写磁盘的线程数，这个参数需要占总核数的50%

num.relica.fetchers=1 ：副本拉取线程数，这个参数占总核数50%的1/3

num.network.threads=3 ：数据传输线程数，这个参数占总核数50%的2/3

还有其他线程，但这三个时间最长

建议 32 个 cores：24个线程分给这三个，另外8个分给其他

**网络选择**

网络带宽= 峰值吞吐量 20M/s * 8=160M，即千兆带宽

- 网络带宽(Mbps) = M/s * 8

# 问题总结

9988占用端口9092：修改bin/kafka-run-class.sh

[client fails trying to bind to jmx port already in use by server by edi-bice · Pull Request #1983 · apache/kafka (github.com)](https://github.com/apache/kafka/pull/1983/commits/2c5d40e946bcc149b1a9b2c01eced4ae47a734c5)
