# Modul-9-Tutorial-A-Subscriber

## Understanding subscriber and message broker

### What is AMQP?

AMQP stands for Advanced Message Queuing Protocol. It is an open standard protocol used by applications to communicate through a message broker. In this tutorial, AMQP is used so the subscriber can connect to RabbitMQ and receive events from the `user_created` queue. With AMQP, the publisher and subscriber do not need to communicate directly because messages are sent through the broker first.

### What does `guest:guest@localhost:5672` mean?

In the URL `amqp://guest:guest@localhost:5672`, the first `guest` is the username used to authenticate to RabbitMQ. The second `guest` is the password for that username. The `localhost:5672` part means the subscriber connects to a RabbitMQ server running on the same machine, using port `5672`, which is the default port for AMQP communication.

## Simulation slow subscriber

![Slow subscriber chart](assets/slow-subscriber-chart.png)

In this experiment, the subscriber is intentionally slowed down by adding a one-second delay before each message is processed. When the publisher is run several times quickly, it can send messages faster than the subscriber can consume them. Because of that, RabbitMQ temporarily stores the unprocessed messages in the `user_created` queue.

The total number of queued messages depends on how many times the publisher is executed and how fast the subscriber can process messages at that moment. In my chart, the queue first reached 10 messages, then gradually went back to 0 after the subscriber processed them. After running the publisher again, the queue rose to 5 messages, then returned to 0 again. This happened because one publisher run sends five messages, while the slow subscriber consumes them one by one with a delay.

This shows why a message broker is useful in an event-driven architecture. The publisher does not need to wait for the slow subscriber to finish processing every request. RabbitMQ becomes a buffer between them, so incoming events can wait safely in the queue until the subscriber is ready to process them.

## Reflection and running at least three subscribers

![RabbitMQ chart with three subscribers](assets/slow-subsriber-chart-with-3-console-open.png)

![Subscriber console 1](assets/subscriber-console-1.png)

![Subscriber console 2](assets/subscriber-console-2.png)

![Subscriber console 3](assets/subscriber-console-3.png)

In this experiment, I opened three subscriber consoles at the same time. RabbitMQ showed 3 connections, 3 channels, 3 consumers, and 1 queue, which means all three subscriber processes were connected to the same `user_created` queue. When the publisher was executed quickly, the total queued messages rose to 10, then returned to 0 after the messages were processed.

The queue decreased faster than the previous slow-subscriber experiment because the messages were distributed to three subscribers instead of only one. Each subscriber still has a one-second delay, but RabbitMQ can deliver different messages to different consumers, so the total processing work is shared. The message rate chart also shows a short spike, with publish and acknowledgement activity rising during the burst and then going back to 0.00/s after the queue became empty.

From the publisher code, one improvement is to avoid hardcoding repeated event data and create the messages from a collection or helper function. From the subscriber code, one useful improvement is to make the AMQP URL configurable through an environment variable instead of hardcoding `amqp://guest:guest@127.0.0.1:5672`. Another improvement is to configure message prefetch, for example one unacknowledged message per subscriber, so RabbitMQ distributes work more fairly when subscribers are slow.

## Bonus: Simulation slow subscriber on cloud

![Cloud slow subscriber chart](assets/bonus-cloud-slow-subscriber-chart.png)

In the cloud experiment, RabbitMQ was running on a Google Cloud VM while the publisher and subscriber connected to it through the VM public IP. The subscriber still used the one-second processing delay, so it could not consume messages as fast as the publisher sent them. In the RabbitMQ cloud chart, the total queued messages rose to 10, stayed there briefly, and then went back down to 0 as the subscriber processed the messages one by one.

The chart also showed 5 connections, 5 channels, 1 queue, and 1 consumer. The most important part is that there was only 1 consumer, so all messages had to be processed by a single slow subscriber. That is why the queue could build up to 10 messages before eventually returning to 0. The message rate chart also showed a short burst around 10-11 messages per second, then returned to 0.00/s after the cloud broker had no more messages to deliver or acknowledge.