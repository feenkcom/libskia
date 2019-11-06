#!/bin/bash

LIBRARY_NAME=Skia

if [[ $TRAVIS_COMMIT_MESSAGE == *"[skip deploy]"* ]]
then
	echo "Skipping deploy stage"
else	
	mkdir -p ~/.ssh/
	touch ~/.ssh/config
	echo -e "Host *\n\tStrictHostKeyChecking no\n" >> ~/.ssh/config
	openssl aes-256-cbc -K $encrypted_11fc2ba54d64_key -iv $encrypted_11fc2ba54d64_iv -in deploy_key.enc -out deploy_key -d
	eval "$(ssh-agent -s)"
	chmod 600 deploy_key
	ssh-add deploy_key
	ssh -i deploy_key $FEENK_CLOUD pwd

	cd target/release

	if [[ $TRAVIS_OS_NAME == "osx" ]]; then
		scp lib$LIBRARY_NAME.dylib $FEENK_CLOUD:/var/www/html/$LIBRARY_NAME/osx/development/x86_64/lib$LIBRARY_NAME.dylib
	fi
	if [[ $TRAVIS_OS_NAME == "linux" ]]; then
		scp lib$LIBRARY_NAME.so $FEENK_CLOUD:/var/www/html/$LIBRARY_NAME/linux/development/x86_64/lib$LIBRARY_NAME.so
	fi
	
	if [[ $TRAVIS_OS_NAME == "windows" ]]; then
		scp $LIBRARY_NAME.dll $FEENK_CLOUD:/var/www/html/$LIBRARY_NAME/windows/development/x86_64/lib$LIBRARY_NAME.dll
	fi
	
	ssh-agent -k
fi
