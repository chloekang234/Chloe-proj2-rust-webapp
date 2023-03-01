# Individual Project2: A functional rust microservice web application using spotify API

## Objectvices of Project

In Project 2, the purpose is to build a functional microservice rust web application deployed automatically to Kubernetes cluster. Kubernetes is a powerful tool for managing containerized applications at scale, providing a flexible and robust platform for deploying and managing applications in the cloud. I built a simple Microservice for music search app that can search songs with the keyword using rust actix web and Spotify API. There are following 4 microservice routes, `/`, `/search?q=keyword`, `/health`, `/version`:

A. / that turns a welcome page
B. /search?q={keyword} that returns search results depending on the keyword
C. /health that returns a 200 status code
D. /version that returns the version of the service


## Project Structure

## Preparation
1. `cargo init`

      After making the new repository, run the command `cargo init` to to create a new Rust package or project. This command creates a new directory with       the necessary files and folders for a Rust package, including the Cargo.toml file, which contains metadata about the package, and a src folder,           which contains the Rust source code for the package.
  
2. Dependencies
      <img width="1013" alt="Screenshot 2023-03-01 at 10 26 30 AM" src="https://user-images.githubusercontent.com/104106034/222185315-93c32434-9943-4e35-8612-c19b84d3705a.png">
  
      In cargo.toml under dependencies section, list the dependencies and dependencies that your Rust project requires to compile and run.
      
3. Dockerfile
      
      <img width="670" alt="Screenshot 2023-03-01 at 10 32 47 AM" src="https://user-images.githubusercontent.com/104106034/222186567-886b2996-35b5-40ea-bffe-2496f7cd97a2.png">
      Set up Dockerfile to containerization for App music_app
      
4. Makefile

      <img width="592" alt="Screenshot 2023-03-01 at 10 35 42 AM" src="https://user-images.githubusercontent.com/104106034/222187277-6d52abd1-e5e5-4b93-ad77-76840291d01f.png">
      Set up Makefile to boost the running time by containing a set of instructions or rules that define how to compile and build the project.

5. main.rs
      Inside **get_access_tocken()** function in main.rs, client_id and client_secret should be defined by each user from Spotify API.
      
      <img width="757" alt="Screenshot 2023-03-01 at 10 47 24 AM" src="https://user-images.githubusercontent.com/104106034/222190686-19348fa8-d84b-492e-9686-c48050102189.png">
      
      How to get Spotify client id and client secret: https://www.thedataschool.co.uk/penny-richmond/howtogetdataoffspotify
      The link for API token: https://developer.spotify.com/console/get-search-item/?q=Muse&type=track&market=US&limit=5&offset=5&include_external=


## Run Microservice

Run `cargo run` to execute the actix web locally.

- welcome page
<img width="806" alt="welcome" src="https://user-images.githubusercontent.com/104106034/222182425-92190c15-d4bd-47f7-9380-4cbd8675fbb8.png">

- version check
<img width="803" alt="version" src="https://user-images.githubusercontent.com/104106034/222182349-ac4e7283-5c73-4c64-a15c-26731e83b0d9.png">

- search keyword with bts
<img width="1097" alt="search-1" src="https://user-images.githubusercontent.com/104106034/222181839-eb6289cd-8c27-46f8-9fa7-26fce5407189.png">

- search keyword with love

<img width="1098" alt="search-2" src="https://user-images.githubusercontent.com/104106034/222182270-5b8ee0ee-6a59-4c9a-82ce-6031a3971e83.png">

## Deploy the app using automatic deployment platform 
A. AWS App Runner
    https://ndh6feck7k.us-east-1.awsapprunner.com/

B. Minikube

  1. Push container to DockerHub (Optional): i.e. docker build -t <hub-user>/<repo-name>[:<tag>] and docker push <hub-user>/<repo-name>:<tag>
     
      My example:
      ```
      docker login -u yk234
      docker built -t yk234/rust-mini
      docker push yk234/rust-mini
      ``` 
  
  ![image](https://user-images.githubusercontent.com/104106034/222196104-76438d31-ca1e-4985-8cac-21dc716a90a9.png)
  ![image](https://user-images.githubusercontent.com/104106034/222198190-5bf31f56-1606-4a8d-abfc-396bc50af1d1.png)

  2. `minikube start`

  3. `minikube dashboard --url`

  4. Hover over link and "follow link"
  
  <img width="1431" alt="image" src="https://user-images.githubusercontent.com/104106034/222198494-1e19605c-3894-44e2-bb03-397713aac3ce.png">

  5. Create a deployment: `kubectl create deployment hi-minikube --image=registry.hub.docker.com/yk234/rust-mini`

  6. View deployment: `kubectl get deployments`

  7. Create service and expose it: `kubectl expose deployment hi-minikube --type=LoadBalancer --port=8080`

  8. View services: `kubectl get service hi-minikube`

  9. `minikube service hi-minikube --url`

  10. Curl web service: i.e. curl http://127.0.0.1:63167
  
  11. Cleanup
      
      ```
      kubectl delete service hello-fastapi
      kubectl delete deployment hello-fastapi
      minikube stop
      ```
