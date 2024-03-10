# Services

## Adding a New Service
To register a service with the gateway to make it available for use from the [Tulsa Web Devs UG Site](https://tulsawebdevs.org), perform the following steps:
- add an entry to `gateway_config.toml` in the following format:
```toml
[services.name] # where 'name' is a unique kebab-case name for your service
path="/myservice" # unique path the frontend will make a request to at /api[path]
host="https://my-service.default.svc.cluster.local" # the host for the service
port=8080 # the target port for your service
```
- add an entry for your service in the list below with a brief description of your service and a link to the documentation

**PR's that do not include documentation of the service will not be merged**

- your service will receive any payload/query params from the initial client request as well as a JWT containing a basic user object with the following schema:
```json
{
  "user":{
    "username":"github_username",
    "metadata": {
      //General User Data TBD
      "services": {
        "your-service-name": {
          //User metadata for your service - 500B max
        }
      }
    }
  } 
}
```
- To update user metadata, make a POST request to `https://tulsawebdevs.org/api/userdata` with a JSON payload in the following schema:
 ```json
{
  "username":"github_username",
  "metadata": {
    // User metadata for your service - 500B max
  }
}
```
- The API will validate your request, attempt to write the user data if valid,  and return a success or failure status. 
*TBD: Auth/Validation/Rate Limit rules for this - will likely have a token endpoint and require a bearer token or similar for requests*

## Registered Services
*Use the following basic format `[repo link](https://myservicelocation)` - Service Name / short description

-[Tulsa Web Devs UG Site](https://tulsawebdevs.org) - The Frontend Static Site that uses this API Gateway
