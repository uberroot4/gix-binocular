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

    const produceMsg = async () => {
        try {
            // Action.ReadDir("web_dir/.git")
            sendAction({ReadDir: msg});
        } catch (err) {
            console.error("Error calling consumer: ", err);
        }
    };

    const stopConsumer= async () => {
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
                <button onClick={startConsumer} disabled={consumerStarted}>Start Consumer</button>
                <button onClick={produceMsg} disabled={!consumerStarted}>Produce</button>
            </div>
            <p>
                <button onClick={stopConsumer} disabled={!consumerStarted}>Stop Consumer</button>
            </p>
        </>
    )
}

export default ProducerConsumer