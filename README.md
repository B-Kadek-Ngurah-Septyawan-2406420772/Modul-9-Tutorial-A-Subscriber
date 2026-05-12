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
