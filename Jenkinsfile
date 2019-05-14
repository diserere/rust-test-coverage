G_giturl = "git@github.com:diserere/rust-test-coverage.git"
G_gitcred = "diserere_on_github"
//~ G_container = "alanin/container:latest"
G_container = "alanin/container:kcov"
//G_container = "container-kcov:optimized"
G_buildstatus = "NotSet"
G_teststatus = "NotSet"
G_rustfmtstatus = "NotSet"
C_PROJECT = "NotSet"
C_COMMITER = "NotSet"
C_HASH = "NotSet"
C_TEXT = "NotSet"

//~ weeklyBuildEnabled = "NotSet"

//FEATURES_LIST = nodead main detailed
//FEATURES_LIST = ''
//~ FEATURES_LIST = 'nodead '
FEATURES_LIST = 'nodead pass'
//~ FEATURES_LIST = 'detailed '
//~ FEATURES_LIST = 'main '
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


echo 'currentBuild.getBuildCauses(): ' + currentBuild.getBuildCauses()

def isBuildTimerTriggered(build) {
    buildCauseFiltered = build.getBuildCauses('hudson.triggers.TimerTrigger$TimerTriggerCause')
    !buildCauseFiltered.toString().equals("[]") 
}

def isBuildSCMTriggered(build) {
    buildCauseFiltered = build.getBuildCauses('jenkins.branch.BranchEventCause')
    !buildCauseFiltered.toString().equals("[]") 
}

def isBuildSucceed(build) {
    build.result.toString().equals("SUCCESS")
}

void setBuildStatus(String message, String state) {
  step([
      $class: "GitHubCommitStatusSetter",
//      reposSource: [$class: "ManuallyEnteredRepositorySource", url: "https://github.com/my-org/my-repo"],
//      contextSource: [$class: "ManuallyEnteredCommitContextSource", context: "ci/jenkins/build-status"],
      contextSource: [$class: "ManuallyEnteredCommitContextSource", context: "continuous-integration/jenkins/branch"],
//      errorHandlers: [[$class: "ChangingBuildStatusErrorHandler", result: "UNSTABLE"]],
      statusResultSource: [ $class: "ConditionalStatusResultSource", results: [[$class: "AnyBuildResult", message: message, state: state]] ]
  ]);
}

def triggers = []
if (isBuildSCMTriggered(currentBuild)) {
    triggers << cron('H/3 * * * *') // every 3 minutes
}

def setTriggers(triggers) {
    properties (
        [
            pipelineTriggers(triggers)
        ]
    )
}

echo 'triggers: ' + triggers


//~ prevBuildCauseFiltered = currentBuild.getPreviousBuild().getBuildCauses('hudson.model.Cause$UpstreamCause')
//~ curBuildCauseFiltered = currentBuild.getBuildCauses('hudson.model.Cause$UpstreamCause')
prevBuildCauseFiltered = currentBuild.getPreviousBuild().getBuildCauses('hudson.triggers.TimerTrigger$TimerTriggerCause')
curBuildCauseFiltered = currentBuild.getBuildCauses('hudson.triggers.TimerTrigger$TimerTriggerCause')
echo "prevBuildCauseFiltered: " + prevBuildCauseFiltered.toString()
echo "curBuildCauseFiltered: " + curBuildCauseFiltered.toString()



/* Set flag for weekly build if current build is timer-triggered, and
 * previous build was successful and was not timer-triggered
 * (means there were SCM changes after last weekly build) 
 */
echo 'isBuildTimerTriggered(currentBuild): ' + isBuildTimerTriggered(currentBuild)
echo 'isBuildSCMTriggered(currentBuild): ' + isBuildSCMTriggered(currentBuild)
echo 'isBuildTimerTriggered(currentBuild.getPreviousBuild()): ' + isBuildTimerTriggered(currentBuild.getPreviousBuild())
echo 'isBuildSucceed(currentBuild.getPreviousBuild()): ' + isBuildSucceed(currentBuild.getPreviousBuild())
weeklyBuildEnabled = false;
if (
    isBuildTimerTriggered(currentBuild) &&
    isBuildSucceed(currentBuild.getPreviousBuild()) &&
    !isBuildTimerTriggered(currentBuild.getPreviousBuild())
) {
    weeklyBuildEnabled = true
}
echo 'weeklyBuildEnabled = ' + weeklyBuildEnabled

/////* Abort build if it is timer-triggered but flag is not set */
////if ( isBuildTimerTriggered(currentBuild) ) {
    ////if ( !weeklyBuildEnabled ) {
        ////echo 'Aborting build...'
        ////currentBuild.setDescription("Weekly kcov build: aborted due to no changes since last run")
        ////currentBuild.result = 'ABORTED'
//////        currentBuild.result = 'SUCCESS'
        ////setBuildStatus("Build aborted due to no changes", "SUCCESS");
        ////return
    ////}
////}
        
                    //~ script {
                        //~ C_TEXT = "WEEKLY BUILD: code coverage against commit " + C_HASH + " commented '" + C_TEXT + "'"
                        //~ currentBuild.setDescription(C_TEXT)
                        //~ echo C_TEXT
                    //~ }


//~ echo "curBuildCause out of: " + currentBuild.getBuildCauses().toString()

pipeline {

//    triggers { cron('H */4 * * 1-5') }
    //~ triggers { cron('H/5 * * * *') }

    //~ triggers { cron('H/2 * * * *') }
    //~ triggers { cron('H H/6 * * *') }
    //~ triggers { cron('H H(0-5) * * 6') }
    //~ triggers { upstream 'rust-test-coverage-runner' }

    //~ triggers { cron('H/15 * * * *') }
    //~ triggers { pollSCM('H/10 * * * *') }
//    triggers { cron('TZ=Europe/Moscow\n H/5 * * * *') }
//    triggers {cron '''
//        TZ=Europe/Moscow
//        H/5 * * * *
//    '''}
//
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

                    C_AUTHOR = sh (script: 'git show -s --format=%an ${GIT_COMMIT}',returnStdout: true).trim()
                    C_COMMITER = sh (script: 'git show -s --format=%cn ${GIT_COMMIT}',returnStdout: true).trim()
                    C_HASH = sh (script: 'git show -s --format=%h ${GIT_COMMIT}',returnStdout: true).trim()
                    if ( weeklyBuildEnabled ) {
                        C_TEXT = "Weekly kcov build: use commit " + C_HASH
                        echo "C_TEXT: " + C_TEXT
                    } else {
                        C_TEXT = sh (script: 'git show -s --format=%s ${GIT_COMMIT}',returnStdout: true).trim()
                    }
                    C_PROJECT = G_giturl.substring(15,G_giturl.length()-4)
                    C_GITURL = sh (script: 'echo ${GIT_URL}',returnStdout: true).trim()
                    C_GITCOMMIT = sh (script: 'echo ${GIT_COMMIT}',returnStdout: true).trim()
/*                    
                    echo "prevBuildRes: " + currentBuild.getPreviousBuild().result
                    echo "prevBuildTm: " + currentBuild.getPreviousBuild().timeInMillis.toString()
                    echo "currentBuildTm: " + currentBuild.timeInMillis.toString()
                    echo "Time diff: " + (currentBuild.timeInMillis - currentBuild.getPreviousBuild().timeInMillis)
                    
                    echo "curBuildCause: " + currentBuild.getBuildCauses().toString()
                    echo "curBuildCauseFiltered: " + currentBuild.getBuildCauses('hudson.triggers.TimerTrigger$TimerTriggerCause').toString()
                    //~ prevBuildCauseFiltered = currentBuild.getPreviousBuild().getBuildCauses('hudson.triggers.TimerTrigger$TimerTriggerCause')

                    prevBuildCauseFiltered = currentBuild.getPreviousBuild().getBuildCauses('hudson.model.Cause$UpstreamCause')
                    curBuildCauseFiltered = currentBuild.getBuildCauses('hudson.model.Cause$UpstreamCause')

                    echo "prevBuildCauseFiltered: " + prevBuildCauseFiltered.toString()
                    echo "curBuildCauseFiltered: " + curBuildCauseFiltered.toString()
                    
                    if ( !prevBuildCauseFiltered.toString().equals("[]") ) {
                        echo "prevBuildCauseFiltered Yes"
                    } else {
                        echo "prevBuildCauseFiltered No"
                    }
                        
                    
                    echo "curBuildDisplayName: " + currentBuild.displayName.toString()
                    echo "curBuildDescription: " + currentBuild.description.toString()

                    weeklyBuildEnabled = false;
                    if (
                            currentBuild.getPreviousBuild().result.toString().equals("SUCCESS") && 
                            //~ currentBuild.timeInMillis - currentBuild.getPreviousBuild().timeInMillis < 300000 
                            !curBuildCauseFiltered.toString().equals("[]") &&
                            prevBuildCauseFiltered.toString().equals("[]")
                        ) {
                            weeklyBuildEnabled = true
                        }
                    
                    echo 'weeklyBuildEnabled = ' + weeklyBuildEnabled
                    if ( !curBuildCauseFiltered.toString().equals("[]") &&
                        !weeklyBuildEnabled ) {
                        try {
                            autoCancelled = true
                            build.doStop()
                        } catch (e) {
                            if (autoCancelled) {
                                currentBuild.result = 'SUCCESS'
                            }
                        }
                            
                        //~ currentBuild.result = 'ABORTED'
                        currentBuild.result = 'SUCCESS'
                        //~ autoCancelled = true
                        //~ return
                    }

                    //C_descr = "Nightly build: " + currentBuild.description.toString()
                    //echo C_descr
                    //currentBuild.getPreviousBuild().getDescription()
*/
                    sh 'echo "Try to find cargo..."'
                    sh 'which cargo'
                    
/* New */
                    ////prevBuildCauseFiltered = currentBuild.getPreviousBuild().getBuildCauses('hudson.triggers.TimerTrigger$TimerTriggerCause')
                    ////curBuildCauseFiltered = currentBuild.getBuildCauses('hudson.triggers.TimerTrigger$TimerTriggerCause')
                    ////echo "prevBuildCauseFiltered: " + prevBuildCauseFiltered.toString()
                    ////echo "curBuildCauseFiltered: " + curBuildCauseFiltered.toString()



                    /////* Set flag for weekly build if current build is timer-triggered, and
                     ////* previous build was successful and was not timer-triggered
                     ////* (means there were SCM changes after last weekly build) 
                     ////*/
                    ////echo 'isBuildTimerTriggered(currentBuild): ' + isBuildTimerTriggered(currentBuild)
                    ////echo 'isBuildTimerTriggered(currentBuild.getPreviousBuild()): ' + isBuildTimerTriggered(currentBuild.getPreviousBuild())
                    ////echo 'isBuildSucceed(currentBuild.getPreviousBuild()): ' + isBuildSucceed(currentBuild.getPreviousBuild())
                    ////weeklyBuildEnabled = false;
                    ////if (
                        ////isBuildTimerTriggered(currentBuild) &&
                        ////isBuildSucceed(currentBuild.getPreviousBuild()) &&
                        ////!isBuildTimerTriggered(currentBuild.getPreviousBuild())
                    ////) {
                        ////weeklyBuildEnabled = true
                    ////}
                    ////echo 'weeklyBuildEnabled = ' + weeklyBuildEnabled

                    /* Abort build if it is timer-triggered but flag is not set */
                    if ( isBuildTimerTriggered(currentBuild) ) {
                        if ( !weeklyBuildEnabled ) {
                            echo 'Aborting build...'
                            currentBuild.setDescription("Weekly kcov build: aborted due to no changes since last run")
                            //currentBuild.result = 'ABORTED'
                    //        currentBuild.result = 'SUCCESS'
                            //~ try {
                                //~ error("Manually aborted due to no changes")
                                currentBuild.result = 'ABORTED'
                                echo 'currentBuild.result: ' + currentBuild.result
                                setBuildStatus("Build aborted due to no changes", "SUCCESS");
                                sh 'exit'
                                //~ sh 'might fail'
                            //~ } catch (err) {
                                //~ echo "Caught: ${err}"
                                //~ currentBuild.result = 'ABORTED'
                                //~ //currentBuild.result = 'SUCCESS'
                                //~ //currentBuild.result = 'FAILURE'
                                //~ setBuildStatus("Build aborted due to no changes", "SUCCESS");
                            //~ }
                            
                            //~ //return
                            //~ throw err
                        }
                    }
                    

                    echo 'triggers: ' + triggers
                    echo 'Setting triggers..'
                    setTriggers(triggers)

                    
                }
            }
        }

        stage('Build') {
            steps {
//                input message: 'Whahaha'
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

//*
        stage('Test coverage') {

            //~ when {
                //~ triggeredBy "TimerTrigger"
            //~ }

            when {
                allOf {
                    triggeredBy 'TimerTrigger'
                    expression { return weeklyBuildEnabled }
                    
                    //~ triggeredBy 'UpstreamCause'
                    
                    //~ environment name: 'weeklyBuildEnabled', value: 'true'
                    //~ equals expected: true, actual: weeklyBuildEnabled
                    //~ equals expected: 2, actual: currentBuild.number
                }
            }

            
            steps {
                dir('.') {
//                    input message: 'Whahaha'
                    Cargo86_64cov('no32')
                }
            }
            post {
                success {
                    script{G_teststatus = "success"}
                    step([
                        $class: 'CoberturaPublisher', 
                        autoUpdateHealth: false, 
                        autoUpdateStability: false, 
                        coberturaReportFile: 'target/cov/kcov-merged/cobertura.xml', 
                        failUnhealthy: false, 
                        failUnstable: false, 
                        maxNumberOfBuilds: 0, 
                        onlyStable: false, 
                        sourceEncoding: 'ASCII', 
                        zoomCoverageChart: false
                    ])
                    publishHTML([
                        allowMissing: false, 
                        alwaysLinkToLastBuild: false, 
                        keepAll: false, 
                        keepAll: true, 
                        reportDir: 'target/cov/', 
                        reportFiles: 'index.html', 
                        reportName: 'Last kcov report', 
                        reportTitles: 'Code coverage report (by kcov) for ${display_name} build ${BUILD_NUMBER} triggered by ${C_COMMITER} commit ${C_HASH} in ${JOB_NAME}'
                    ])
                }
                failure {script{G_teststatus = "failure"}}
            }
        }
//*/

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
/*
            step([
                $class: 'CoberturaPublisher', 
                autoUpdateHealth: false, 
                autoUpdateStability: false, 
                coberturaReportFile: 'target/cov/kcov-merged/cobertura.xml', 
                failUnhealthy: false, 
                failUnstable: false, 
                maxNumberOfBuilds: 0, 
                onlyStable: false, 
                sourceEncoding: 'ASCII', 
                zoomCoverageChart: false]
            )
            publishHTML([
        allowMissing: false, 
                alwaysLinkToLastBuild: false, 
//                keepAll: false, 
                keepAll: true, 
                reportDir: 'target/cov/', 
                reportFiles: 'index.html', 
                reportName: 'Last kcov report', 
                reportTitles: ''
        ])
//*/
        }

    }
}
