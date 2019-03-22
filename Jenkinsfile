G_giturl = "git@github.com:diserere/rust-test-coverage.git"
G_gitcred = "diserere_on_github"
//G_container = "alanin/container:latest"
G_container = "container-kcov"
G_buildstatus = "NotSet"
G_teststatus = "NotSet"
G_rustfmtstatus = "NotSet"
C_PROJECT = "NotSet"
C_COMMITER = "NotSet"
C_HASH = "NotSet"
C_TEXT = "NotSet"

//FEATURES_LIST = nodead main detailed
FEATURES_LIST = 'nodead '
// build_features = "--features 'nodead main detailed' "
build_features = "--features '${FEATURES_LIST}' "

def Cargo86_64build(bits) {
    sh 'cargo clean'
    sh 'cargo build'
    if (bits != "no32") {
        sh 'OPENSSL_DIR="/ssl/" cargo build --target=i686-unknown-linux-gnu'
    }
}

def Cargo86_64test(bits) {
    sh 'cargo test'
    if (bits != "no32") {
        sh 'OPENSSL_DIR="/ssl/" cargo test --target=i686-unknown-linux-gnu'
    }
}

def Cargo86_64cov(bits) {
    sh 'cargo clean'
    sh "cargo kcov ${build_features}"
//    sh "cargo kcov --features 'nodead main detailed' "
    if (bits != "no32") {
        sh 'OPENSSL_DIR="/ssl/" cargo build --target=i686-unknown-linux-gnu --features "nodead main detailed"'
    }
}

// Deploy channel
//DiscordURL = "https://discordapp.com/api/webhooks/496992026932543489/4exQIw18D4U_4T0H76bS3Voui4SyD7yCQzLP9IRQHKpwGRJK1-IFnyZLyYzDmcBKFTJw"
DiscordURL = "https://discordapp.com/api/webhooks/558405801392209920/QJb6F6yJTu9mL1dTvDelyzPylSHZaciNqHi9m3AyhkHX9XAN5wUbp7QHOUkqqg_34FKw"

pipeline {
    agent {
        docker {
            image G_container
            args '--network proxy_nw --security-opt seccomp=unconfined'
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
            steps {
                Cargo86_64build('no32')
            }
            post {
                success {script{G_buildstatus = "success"}}
                failure {script{G_buildstatus = "failure"}}
            }
        }

        stage('Test') {
            steps {
                dir('.') {
                    Cargo86_64test('no32')
//                    archiveArtifacts artifacts: 'target/release/libtvm.so', onlyIfSuccessful: true
//                    archiveArtifacts artifacts: 'target/i686-unknown-linux-gnu/release/libtvm.so', onlyIfSuccessful: true
                }
            }
            post {
                success {script{G_teststatus = "success"}}
                failure {script{G_teststatus = "failure"}}
            }
        }

        stage('Test coverage') {
            steps {
                dir('.') {
//                    input message: 'Whahaha'
                    Cargo86_64cov('no32')
//                    archiveArtifacts artifacts: 'target/release/libtvm.so', onlyIfSuccessful: true
//                    archiveArtifacts artifacts: 'target/i686-unknown-linux-gnu/release/libtvm.so', onlyIfSuccessful: true
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
            step([  $class: 'CoberturaPublisher', 
                    autoUpdateHealth: false, 
                    autoUpdateStability: false, 
                    coberturaReportFile: '**/coverage.xml', 
                    failUnhealthy: false, 
                    failUnstable: false, 
                    maxNumberOfBuilds: 0, 
                    onlyStable: false, 
                    sourceEncoding: 'ASCII', 
                    zoomCoverageChart: false])
        }

    }
}
