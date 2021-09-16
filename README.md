# hyper-demo-web

### Start
При запуске сервера все основные параметры передаются в виде аргументов, или переменных  
окружения. Если в момент старта приложения будет обнаружен аргумент `--configmap`, или  
переменная `CONFIGMAP`, то сервер будет использовать параметры оттуда (если сможет ее  
прочитать и сериализовать). При это главенство параметров из аргументов и переменных  
над параметрами указанными в `configmap` будет сохранено.

### Expanding
Этот шаблон позволяет запускать сразу несколько серверов, но это потребует дополнительной  
реализации сервисов которые будут обслуживать `1+n` сервера.
```rust
#[tokio::main]
async fn main() -> Result<()> {
    // ...
    // main server realization
    
    // additional server
    let new_address = format!("{}:{}", HOST, PORT).parse().unwrap();

    let new_service = make_service_fn(move |_| {
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                new_response(req)
            }))
        }
    });

    let new_server = Server::bind(&addr).serve(main_service);
    let new_graceful =  new_server.with_graceful_shutdown(
        shutdown_signal()
    );
    
    join![server, new_server];
    Ok(())
}
```

Что бы сделать 