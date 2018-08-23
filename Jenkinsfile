pipeline {
  agent { kubernetes {
      label 'myrustpod'
      defaultContainer 'jnlp'
      yaml """
apiVersion: v1
kind: Pod
metadata:
  labels:
    some-label: some-label-value
spec:
  containers:
  - name: rust
    image: rust
    command:
    - cat
    tty: true
  - name: docker
    image: docker
    command:
    - cat
    tty: true
    volumeMounts:
    - mountPath: /var/run/docker.sock
      name: dockersock
  volumes:
  - hostPath:
      path: /var/run/docker.sock
      type: ""
    name: dockersock
"""
    }
  }
  stages {
    stage('git') {
      steps {
          git "https://github.com/antweiss/docker-dummy.git"
        }
      }
    stage('rust') {
      steps {
        container('rust'){
          sh 'cargo build --release'
        }
      }
    }
    stage('docker') {
        steps{
            container('docker'){
                script{
                    image = docker.build("otomato/oto-tiny-http:${BUILD_NUMBER}")
                    docker.withRegistry('', 'dockerlogin'){
                        image.push()
                    } 
                }
            }
        }
    }
  }
}
