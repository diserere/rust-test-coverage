G_giturl = "git@github.com:diserere/rust-test-coverage.git"
//G_gitcred = "LaninSSHgit"
//G_container = "alanin/container:latest"
G_container = "rust"
G_buildstatus = "NotSet"
G_teststatus = "NotSet"
G_rustfmtstatus = "NotSet"
C_PROJECT = "NotSet"
C_COMMITER = "NotSet"
C_HASH = "NotSet"
C_TEXT = "NotSet"

def Cargo86_64build(bits) {
    sh 'cargo clean'
    sh 'cargo build --release --features ci_run'
    if (bits != "no32") {
        sh 'OPENSSL_DIR="/ssl/" cargo build --release --features ci_run --target=i686-unknown-linux-gnu'
    }
}
def Cargo86_64test(bits) {
    sh 'cargo test --release --features ci_run'
    if (bits != "no32") {
        sh 'OPENSSL_DIR="/ssl/" cargo test --release --features ci_run --target=i686-unknown-linux-gnu'
    }
}

// Deploy chanel
DiscordURL = "https://discordapp.com/api/webhooks/496992026932543489/4exQIw18D4U_4T0H76bS3Voui4SyD7yCQzLP9IRQHKpwGRJK1-IFnyZLyYzDmcBKFTJw"

pipeline {
    agent {
        docker {
            image G_container
//            args '--network proxy_nw'
        }
    }
    environment {
        RUSTFMT_STATUS = "NotSet"
    }
    options {
        buildDiscarder logRotator(artifactDaysToKeepStr: '', artifactNumToKeepStr: '', daysToKeepStr: '', numToKeepStr: '10')
        disableConcurrentBuilds()
        parallelsAlwaysFailFast()
    }
    stages {
        stage('Initialize') {
            steps {
                script {
                    G_gitproject = G_giturl.substring(0,G_giturl.length()-4)
                    properties([[
                        $class: 'GithubProjectProperty',
                        projectUrlStr: G_gitproject
                    ]])
                    G_gitbranch = sh (script: 'echo ${BRANCH_NAME}', returnStdout: true).trim()

                    C_TEXT = sh (script: 'git show -s --format=%s ${GIT_COMMIT}',returnStdout: true).trim()
                    C_AUTHOR = sh (script: 'git show -s --format=%an ${GIT_COMMIT}',returnStdout: true).trim()
                    C_COMMITER = sh (script: 'git show -s --format=%cn ${GIT_COMMIT}',returnStdout: true).trim()
                    C_HASH = sh (script: 'git show -s --format=%h ${GIT_COMMIT}',returnStdout: true).trim()
                    C_PROJECT = G_giturl.substring(15,G_giturl.length()-4)
                    C_GITURL = sh (script: 'echo ${GIT_URL}',returnStdout: true).trim()
                    C_GITCOMMIT = sh (script: 'echo ${GIT_COMMIT}',returnStdout: true).trim()

		    
                    sh 'echo "Try to find cargo..."'
                    sh 'which cargo'
                }
            }
        }

        stage('Build') {
            parallel {
                stage('Build TVM') {
                    steps {
                        dir('tvm') {Cargo86_64build('')}
                    }
                }

                stage('Build ton_block') {
                    steps {
                        dir('ton_block') {Cargo86_64build('')}
                    }
                }
                
                stage('Build poa') {
                    steps {
                        dir('poa') {Cargo86_64build('no32')}
                    }
                }

                stage('Build static abi-lib') {
                    steps {
                        dir('abi-lib/static') {Cargo86_64build('')}
                    }
                }

                stage('Build TON-Node & Utils') {
                    steps {Cargo86_64build('')}
                }
            }
            post {
                success {script{G_buildstatus = "success"}}
                failure {script{G_buildstatus = "failure"}}
            }
        }

        stage('Test') {
            parallel {
                stage('Test TVM') {
                    steps {
                        dir('tvm') {
                            Cargo86_64test('')
                            archiveArtifacts artifacts: 'target/release/libtvm.so', onlyIfSuccessful: true
                            archiveArtifacts artifacts: 'target/i686-unknown-linux-gnu/release/libtvm.so', onlyIfSuccessful: true
                        }
                    }
                }

                stage('Test ton_block') {
                    steps {
                        dir('ton_block') {
                            Cargo86_64test('')
                            archiveArtifacts artifacts: 'target/release/libton_block.so', onlyIfSuccessful: true
                            archiveArtifacts artifacts: 'target/i686-unknown-linux-gnu/release/libton_block.so', onlyIfSuccessful: true
                        }
                    }
                }
                
                stage('Test poa') {
                    steps {
                        dir('poa') {Cargo86_64test('no32')}
                    }
                }

                stage('Test static abi-lib') {
                    steps {
                        dir('abi-lib/static') {Cargo86_64test('')}
                    }
                }
                
                stage('Test TON-Node & Utils') {
                    steps {Cargo86_64test('')}
                }
            }
            post {
                success {script{G_teststatus = "success"}}
                failure {script{G_teststatus = "failure"}}
            }
        }

        stage('RustFmt') {
            steps {
                script{
                    G_rustfmtstatus = sh (script: 'set +e ; cargo fmt --all -- --check', returnStatus: true)
                    if (G_rustfmtstatus) {
                        G_rustfmtstatus = "failure"
                    }
                    else {
                        G_rustfmtstatus = "success"
                    }
                }
            }
        }

        stage('Doc') {
            steps {
                sh "cargo doc"
                // We run a python `SimpleHTTPServer` against
                // /var/lib/jenkins/jobs/<repo>/branches/master/javadoc to
                // display our docs
                step([$class: 'JavadocArchiver',
                      javadocDir: 'target/doc',
                      keepAll: false])
            }
        }
    }
    post {
        always {
            script {
                currentBuild.description = C_TEXT
                string DiscordFooter = "Build duration is " + currentBuild.durationString
                DiscordTitle = "Job ${JOB_NAME} from GitHub " + C_PROJECT
                DiscordDescription = C_COMMITER + " pushed commit " + C_HASH + " by " + C_AUTHOR + " with a message '" + C_TEXT + "'" + "\n" \
                + "Build number ${BUILD_NUMBER}" + "\n" \
                + "Build: **" + G_buildstatus + "**" + "\n" \
                + "Tests: **" + G_teststatus + "**" + "\n" \
                + "Rustfmt: **" + G_rustfmtstatus + "**"
                discordSend description: DiscordDescription, footer: DiscordFooter, link: JOB_DISPLAY_URL, successful: currentBuild.resultIsBetterOrEqualTo('SUCCESS'), title: DiscordTitle, webhookURL: DiscordURL

            }
        }

    }
}
