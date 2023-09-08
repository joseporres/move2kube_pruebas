# Move2kube Sample Application Replatforming

This README provides instructions on how to use [Move2kube](https://move2kube.konveyor.io/) to replatform sample applications from their source code to Kubernetes-ready artifacts.

## Prerequisites

Before you begin, make sure you have the following prerequisites installed:

1. **Move2kube**: Move2kube is the tool we'll use for replatforming. You can install it using Homebrew by running:

    ```shell
    brew tap konveyor/move2kube
    brew install move2kube
    ```

## Replatforming Process

Now that you have the prerequisites installed and the sample applications downloaded, you can start the replatforming process.

### 1. Generate Plan File

First, generate a plan file that specifies how the source code should be transformed into Kubernetes artifacts:

```shell
move2kube plan -s language-platforms
```

### 2. View the Plan

You can view the generated plan file to understand the transformation that will be performed:

```shell
cat m2k.plan
```
### 3. Start the Transformation

Execute the transformation process to convert the source code to Kubernetes-ready artifacts:

```shell
move2kube transform
```

In this step, Move2kube will perform ask you to provide some information about the application. See this video for more information on how to answer these questions form minute 8:10 - 13:10:

[![asciicast](https://asciinema.org/a/423883.svg)](https://www.youtube.com/watch?v=asFooD-1RHs)


### 4. Enter the Project

Navigate to the project directory where the transformed artifacts are located:

```shell
cd <your-project-name>/scripts
```

### 5. Build Docker Images

Run the `builddockerimages.sh` script to build Docker images for the Kubernetes artifacts:

```shell
bash builddockerimages.sh
```

### 6. Push Docker Images (Optional)

If you want to push the Docker images to Docker Hub, you can do so by running the `pushimages.sh` script. Before running the script, make sure to set the `REGISTRY_NAMESPACE` variable to your Docker Hub username if necessary in the pushimages.sh script:

```shell
# Set your Docker Hub username
REGISTRY_NAMESPACE=<docker_hub_user>
```

Then, push the images:
```shell
bash pushimages.sh
```

That's it! You have successfully replatformed the sample application using Move2kube. You can now deploy these Kubernetes artifacts as needed.

Note: Make sure to replace <your-project-name> and <docker_hub_user> with your actual project name and Docker Hub username, respectively, as needed.

After this you can deploy the application using the following command:

```shell
kubectl apply -f <your-project-name>/deploy
```

Make sure to start the minikube cluster before deploying the application.