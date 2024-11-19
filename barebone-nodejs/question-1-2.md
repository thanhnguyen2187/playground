# Question 1.2

You are tasked with designing a scalable and resilient architecture for a
content delivery network (CDN) service. The CDN must efficiently serve a large
number of static assets like images, CSS files, and JavaScript files to users
across the globe.

Design considerations:

- Service Modularization: How would you break down the CDN into individual
  components or services? List the services and explain their functionality.
- Caching Mechanisms: Describe how you would implement caching to ensure fast
  delivery of content. Include strategies for caching at different levels (e.g.,
  edge, origin).
- Redundancy and Failover: Discuss how you would ensure the CDN remains
  available even if some servers fail.
- Data Consistency: Explain how you would ensure that all users receive the most
  up-to-date content, especially when changes are made.

Provide a detailed explanation for each aspect. Use examples where necessary to
illustrate your points.

---

## Service Modularization

I would break down the CDN into the following services:

- Geographical Distribution Service: This service would be responsible for 
  determining which servers are closest to the user and route the requests 
  accordingly.
- Edge Servers: They are placed close to the users and are responsible for 
  actually serving the content to the user.
- Cache Management Services: They take care of managing the cache by populating
  cache entries from the Origin Service and expire cache entries of Edge 
  Servers.
- Origin Service: They are the "source of truth" for the content.

## Caching Mechanisms

Applying the 80-20 rule, I would let the size of the Edge Servers' entries 
to be around 20% of the Origin Service's size. This would allow for a good 
balance between the number of requests and the size of the cache. The Cache 
Management Service would be responsible for updating the cache entries when 
the Origin Service's content changes.

## Redundancy and Failover

As the Edge Servers are user-facing and most likely to be overloaded, I would
ensure that the Edge Servers have backups, and there is a Load Balancer 
before them to handle failover. The same goes for caches, which either 
requires the same mechanism, or a specific technology.

## Data Consistency

As mentioned, the Origin Service is the source of truth, and I would use the 
Cache Management Service to update the cache entries when the Origin Service 
has any changes. This would ensure that all users receive the most up-to-date 
content. There should be some TTL (Time-To-Live) mechanism to ensure that 
users don't get stale content in case Cache Management Service is down.

