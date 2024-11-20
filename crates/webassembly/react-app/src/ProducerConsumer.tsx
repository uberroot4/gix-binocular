import {Action, consumer, producer} from "commits-wasm-web";
import {useState} from "react";

function ProducerConsumer() {
    const [msg, setMsg] = useState("web_repo");

    const startConsumer = async () => {
        try {
            consumer();
        } catch (err) {
            console.error("Error starting consumer: ", err);
        }
    };

    const produceMsg = async () => {
        try {
            // Action.ReadDir("web_dir/.git")
            producer({ReadDir: msg});
        } catch (err) {
            console.error("Error calling consumer: ", err);
        }
    };

    const stopConsumer= async () => {
        try {
            producer("Stop");
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
                <button onClick={startConsumer}>Start Consumer</button>
                <button onClick={produceMsg}>Produce</button>
            </div>
            <p>
                <button onClick={stopConsumer}>Stop Consumer</button>
            </p>
        </>
    )
}

export default ProducerConsumer