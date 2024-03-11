# crawler
A web crawler in Rust.

[![CI](https://github.com/tomfran/crawler/actions/workflows/ci.yml/badge.svg)](https://github.com/tomfran/crawler/actions/workflows/ci.yml)

## Architecture


Here is a schema of the crawler architecture, the main components are: **Workers**, **Dispatcher**

![schema-l.png](misc/schema-l.png#gh-light-mode-only)
![schema-d.png](misc/schema-d.png#gh-dark-mode-only)

The idea is to have multiple workers fetching pages, each one of them parse the HTML content extracting URLs from it, adding them to the Dispatcher queue.

The Dispatcher is in charge of discarding already visited URLs, for this reason 
a **Bloom filter** is used. 

When a URL passes the filter it is added to a **to-visit deque**, which is 
polled by a thread adding it to the **politeness priority queue** component.

The idea of this last component is to respect a website by not flooding it with 
requests. A node is made of **(base_url, next_visit_timestamp)** and the priority is 
given by the second parameter. If a worker finds a node with **next_visit_timestamp** in the future, it waits.
The priority queue contains base urls, there exists a support map 
storing all the pages to visit for a base url.

The complete workflow to visit a new url is then: 
- peek into the priority queue, if the node is not visitable, wait;
- pop the head of the queue, extract a base url from the urls map;
- enqueue the old head with a new timestamp computed as **old + politeness_delay**;
- return the retrieved url.
