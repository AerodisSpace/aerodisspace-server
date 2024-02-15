/// This trait is used to convert the DTOs to the application models and vice versa
trait InterDtoAndApp {
    type App;
    type Dto;

    fn to_app(&self) -> Self::App;
    fn to_dto(&self) -> Self::Dto;
}
