# Modul-9-Tutorial-A-Subscriber

## Understanding subscriber and message broker

### What is AMQP?

AMQP stands for Advanced Message Queuing Protocol. It is an open standard protocol used by applications to communicate through a message broker. In this tutorial, AMQP is used so the subscriber can connect to RabbitMQ and receive events from the `user_created` queue. With AMQP, the publisher and subscriber do not need to communicate directly because messages are sent through the broker first.

### What does `guest:guest@localhost:5672` mean?

In the URL `amqp://guest:guest@localhost:5672`, the first `guest` is the username used to authenticate to RabbitMQ. The second `guest` is the password for that username. The `localhost:5672` part means the subscriber connects to a RabbitMQ server running on the same machine, using port `5672`, which is the default port for AMQP communication.
