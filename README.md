# NEWS Aggregator

## Project Traits

- Documentation 1st [ Interfaces / Traits 1st before actual Code ]
- Example Aggregators ( Types & Examples )
  - Paid / Free / Freemium APIs (HTTP)
    - [GNews API](https://gnews.io)
  - APIs [ UDP (HTTP/3 ?), GRPC (First Hand Citizen when possible) ]
  - RSS Feed
    - Google?
  - Emails
    - SendGrid?
  - ??
- USE-CASES
  - `fetching N news articles`
  - `finding a news article with a specific title or author`
  - `searching by keywords`
  - `include a cache in your API service`
    - See below for [MVP Guidelines]

## MVP Guidelines

- Wrapper around [GNews API] with room to grow via Tilt, Skaffold, Kubernetes, ...
- Simple but Elegant Coding Patterns
- Fast Deployment
- Cache should be deployed via existing technologies
  - Why reinvent the wheel?
