import {sendAction} from "commits-wasm-web";
import {useState} from "react";

function ProducerConsumer() {
    const [msg, setMsg] = useState("web_repo");
    const [consumerStarted, setConsumerStarted] = useState(false);

    const startConsumer = async () => {
        try {
            // consumer();
            sendAction("Start")
            setConsumerStarted(true);
        } catch (err) {
            console.error("Error starting consumer: ", err);
        }
    };

    const readDir = async () => {
        try {
            sendAction({ReadDir: msg});
        } catch (err) {
            console.error("Error calling consumer: ", err);
        }
    };

    const openFile = async () => {
        try {
            sendAction({OpenFile: msg});
        } catch (err) {
            console.error("Error calling consumer: ", err);
        }
    };

    const readFile = async () => {
        try {
            sendAction({ReadFile: msg});
        } catch (err) {
            console.error("Error calling consumer: ", err);
        }
    };
    const metadata = async () => {
        try {
            sendAction({Metadata: msg});
        } catch (err) {
            console.error("Error calling consumer: ", err);
        }
    };

    const stopConsumer = async () => {
        try {
            sendAction("Stop");
            setConsumerStarted(false);
        } catch (err) {
            console.error("Error stopping consumer: ", err);
        }
    };

    const changeMsg = ({target}) => {
        setMsg(target.value)
    }


    return (
        <>
            <h2>Producer Consumer</h2>
            <p>
                {/*<label>*/}
                <input type="text" value={msg} onChange={changeMsg}/>
                {/*</label>*/}
                {/*<input type="submit" value="Submit"/>*/}
            </p>
            <div className="card">
                <p>
                    <button onClick={startConsumer} disabled={consumerStarted}>Start Consumer</button>
                </p>
                <p>
                    <button onClick={readDir} disabled={!consumerStarted}>Action::ReadDir</button>
                    <button onClick={openFile} disabled={!consumerStarted}>Action::OpenFile</button>
                    <button onClick={readFile} disabled={!consumerStarted}>Action::ReadFile</button>
                    <button onClick={metadata} disabled={!consumerStarted}>Action::Metadata</button>
                </p>
            </div>
            <p>
                <button onClick={stopConsumer} disabled={!consumerStarted}>Stop Consumer</button>
            </p>
        </>
    )
}

export default ProducerConsumer