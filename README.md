# mini_actix
rust mini project with actix and deployment on AWS app runner using ECR registry

An annoying web service that repeats what you say to it

## Usage:
The app is deployed using AWS app runner. 
Go to the link- https://kshidxjswv.us-east-1.awsapprunner.com/
![image](https://user-images.githubusercontent.com/110474064/220707001-d12a83ff-26cf-4fe7-bf3f-d1de6f5d19ee.png)

### it displays "Hello World!" by default
![image](https://user-images.githubusercontent.com/110474064/220707225-62eab673-b075-4755-b764-efc8353ddb09.png)

### If you pass it mockingjay/<your string>, it will display that string back to you
![image](https://user-images.githubusercontent.com/110474064/220707527-43b625be-3b26-41b5-b80e-6b7e45816192.png)
![image](https://user-images.githubusercontent.com/110474064/220707648-0415faef-deef-41fb-a3e0-2172dfed8b37.png)
![image](https://user-images.githubusercontent.com/110474064/220707776-b0c66624-6c62-4bca-a1b9-5c40856b2be8.png)

### If you get too annoyed just tell it to stop
![image](https://user-images.githubusercontent.com/110474064/220707976-598cd108-c0c3-4c03-bd9d-353b9e17f92d.png)

## Kubernetes:

## start minikube
![image](https://user-images.githubusercontent.com/110474064/222281785-d26194a4-9089-435f-9bc8-4923fbc0fe14.png)

## cmd to open the dashboard
![image](https://user-images.githubusercontent.com/110474064/222281851-a364b106-854f-4b00-a12f-358e0884a4fd.png)
![image](https://user-images.githubusercontent.com/110474064/222281890-2ca29d30-7fd1-438c-8132-1319dff54515.png)


## create a deployment with the docker image pushed to docker hub
![image](https://user-images.githubusercontent.com/110474064/222281964-305ddae0-5472-4aeb-b9db-461d48998c36.png)

## Check the ip and port that is exposed for the service
![image](https://user-images.githubusercontent.com/110474064/222282066-b6d2374a-dc12-43c7-b152-375c6cec3c41.png)

## access the service url
![image](https://user-images.githubusercontent.com/110474064/222282132-9b96e3a2-4ad9-4a18-8275-7a0273913f1c.png)

## clean up
![image](https://user-images.githubusercontent.com/110474064/222282186-d06da213-132a-4982-bc42-03b89abe24ad.png)

![image](https://user-images.githubusercontent.com/110474064/222282203-0cf7cf62-537e-4043-bb0d-002251c260e3.png)


