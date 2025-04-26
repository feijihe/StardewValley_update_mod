import { useState, useMemo } from "react";
import reactLogo from "./assets/react.svg";
import { Button, Table } from "antd";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { open } from '@tauri-apps/plugin-dialog';

interface Mod {
  id: string; // 唯一标识符
  name: string; // 名称
  version: string; // 版本 
}

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [modsDirectory, setModsDirectory] = useState<string>(''); // 存储目录
  const [mods, setMods] = useState<Mod[]>([]); // 存储mods lis

  async function handleSelectDirectory() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择存储目录'
    });
    
    if (typeof selected === 'string') {
      console.log('选择的目录:', selected);
      setModsDirectory(selected);

      const result = await invoke<Mod[]>('check_mods', { directory: selected }); // 传递目录
      console.log('result: ', result);
      setMods(result);
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

  const columns = useMemo(() => [
    { title: 'ID', dataIndex: 'id', key: 'id' },
    { title: '名称', dataIndex: 'name', key: 'name' },
    { title: '当前版本', dataIndex: 'version', key: 'version' },
    { title: '最新版本', dataIndex: 'latest_version', key: 'latest_version' },
    { title: '操作', key: 'action', render: () => <Button type="primary">下载</Button> }
  ], []);

  return (
    <main className="container">
      <form action="">
        <input type="text" disabled value={modsDirectory} />
        <button type="button" onClick={handleSelectDirectory}>选择目录</button>
      </form>
      <div>
        <Button onClick={handleCheck}>开始检测</Button>
        <Button type="primary">开始下载</Button>
      </div>
      <div>
        <Table 
        dataSource={mods} 
        columns={columns} 
        style={{width:700, margin: 'auto'}} 
        pagination={false} 
        scroll={{ y: 500 }}
        />
      </div>
    </main>
  );
}

export default App;
