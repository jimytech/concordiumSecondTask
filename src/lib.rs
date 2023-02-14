//A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

/// Your smart contract state.
#[derive(Serialize, SchemaType, Clone)]
pub struct State {
    msg: String,
    country_option: String,
}

//#[derive(Serialize, SchemaType)]
//struct InitParameter{
   // msg: String,
    //country_option: String,
//}

#[init(contract = "hello-world-contract")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    //let param: InitParameter = _ctx.parameter_cursor().get()?;
    Ok(State {
        msg: "Hello World".to_string(),
        country_option: "Venezuela".to_string(),
    })
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum ContractError {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
    CountryError,
    HelloError,
}

#[receive(
    contract = "hello-world-contract",
    name = "country",
    parameter = "String",
    error = "ContractError",
    mutable,
)]
fn country<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), ContractError> {
    let param: String = _ctx.parameter_cursor().get()?;
    let state = _host.state_mut();

    ensure!(param != "", ContractError::CountryError);
    state.country_option = param;
    Ok(())
}

#[receive(
    contract = "hello-world-contract",
    name = "hello",
    parameter = "String",
    error = "ContractError",
    mutable,
)]
fn hello<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), ContractError> {
    let param: String = _ctx.parameter_cursor().get()?;
    let state = _host.state_mut();

    ensure!(param != "", ContractError::HelloError);
    state.msg = param;
    Ok(())
}

/// View function that returns the content of the state.
#[derive(Serialize, SchemaType)]
struct HelloView {
    country_option: String,
    msg: String,
}

#[receive(contract = "hello-world-contract", name = "view", return_value = "HelloView")]
fn view<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<HelloView> {
    let state = host.state();
    let msg = state.msg.clone();
    let country_option: String = state.country_option.clone();

    Ok(HelloView{
        msg,
        country_option,
    })
}

/// Test that invoking the `receive` endpoint with the `false` parameter
/// succeeds in updating the contract.
#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    #[test]
    fn hello_works(){           
        let mut _ctx = TestReceiveContext::empty();
        //ctx.set_sender(Address::Account(ACC));

        let msg = "Hello World";
        let parameter = to_bytes(&msg);
        _ctx.set_parameter(&parameter);

        let state: State = State{country_option: "Venezuela".to_string(), msg: "Hello World".to_string()};
        let mut host = TestHost::new(state, TestStateBuilder::new());
        let result = hello(&_ctx, &mut host);

        assert!(result.is_ok());
        assert_eq!(host.state().msg, "Hello World".to_string());
        //assert_eq!(host.state().country_option, "Venezuela".to_string());
        }
    }
