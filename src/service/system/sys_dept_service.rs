pub struct SysDeptService;

impl SysDeptService {

    //使用闭包模拟类，只不过返回值有点冗长，“行为优先的对象”
    pub fn test_closure(num1:i32,num2:i32)->(impl Fn() -> i32,impl Fn() -> i32,impl Fn() -> i32,impl Fn() -> i32) {
        //1：num1和num2相当于是类的构造函数传入的参数

        //2：num3和num4相当于是类的成员变量
        let num3 =100;
        let num4 =200;

        //3：返回的闭包相当于是类的成员函数
        return (Box::new(move||->i32 {
            return num1+num2;
        }),Box::new(move||->i32 {
            return num1-num2;
        }),Box::new(move||->i32 {
            return num1*num2;
        }),Box::new(move||->i32 {
            return num1/num2;
        }));
    }
}