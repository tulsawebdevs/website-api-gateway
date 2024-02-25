# API Gateway 
A simple configurable API gateway for linking microservices to the Tulsa Web Devs User Group site.

Built in Rust and containerized with Docker. 

## ROADMAP
### MVP 0.x
- [ ] Define API
- [ ] Containerize for development
- [ ] Build Config mechanism
- [ ] Build Simple Routing mechanism (JWT auth from front end, all just passed username / basic user metadata from Clerk.io, contents TBD)
*as time allows, build the Clerk interface to back pair of auth and metadata services so that the auth/metadata service is configurable*
- [ ] Containerize for deployment
- [ ] Test
- [ ] Deploy

### Future Development
#### High Priority
- [ ] Add reverse proxy
- [ ] Add auth bypass option for public APIs to config (Auth on by default)
- [ ] Add read/write options to user metadata per service (nested under service name)

#### Medium Priority
- [ ] Make any adjustments needed for horizontal scaling in containers
- [ ] Add load balancer 

#### If Needed
- [ ] Add Redis or other backing service interface for service metadata (separate from auth/Clerk)
