self.addEventListener('error', (error: ErrorEvent) => {
    console.error('error (file-worker)', error)
});

self.addEventListener('message', (e: MessageEvent) => {
    try {
        console.debug('received a message (file-worker)', e.data, e)
    } catch (err) {
        console.error('error processing message (file-worker)', e.data, err)
    }
});