import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { open } from '@tauri-apps/plugin-dialog';

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [modsDirectory, setModsDirectory] = useState<string>(''); // 存储目录

  async function handleSelectDirectory() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择存储目录'
    });
    
    if (typeof selected === 'string') {
      console.log('选择的目录:', selected);
      setModsDirectory(selected);
      // 这里可以调用Rust命令处理目录
    }
  }

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  async function handleCheck() {
    if(!modsDirectory) {
      alert('请先选择目录');
      return; 
    }

    try {
      const result = await invoke('check_mods', { directory: modsDirectory }); // 传递目录
      console.log('检测结果:', result);
      alert(result); // 显示检测结果
    }catch(error) {
      console.error('检测错误:', error);
      alert('检测过程中出错222'); // 显示错误
    }
  }

  return (
    <main className="container">
      <form action="">
        <input type="text" disabled value={modsDirectory} />
        <button type="button" onClick={handleSelectDirectory}>选择目录</button>
      </form>
      <div>
        <button onClick={handleCheck}>开始检测</button>
        <button>开始下载</button>
      </div>
    </main>
  );
}

export default App;
